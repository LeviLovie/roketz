use crossbeam::channel::{Receiver, Sender, unbounded};
use helpers::error::HandleError;
use macroquad::prelude::*;
use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread,
};

use super::{AABB, BVHNode};

#[derive(Debug, Clone, PartialEq)]
pub enum DestructionType {
    Circle(Vec2, f32),
    Point(Vec2),
}

pub struct BVH {
    bounds: AABB,
    root: Arc<Mutex<BVHNode>>,
    max_depth: usize,
    destructions: Sender<DestructionType>,
    destructions_finished: Arc<Mutex<bool>>,
    last_optimize: Arc<Mutex<std::time::Instant>>,
    optimizations: Arc<Mutex<u32>>,
    updated: Arc<Mutex<bool>>,
}

impl BVH {
    pub fn new(width: u32, height: u32, max_depth: usize) -> Self {
        let bounds = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(width as f32, height as f32),
        };

        let (tx, rx) = unbounded();

        let bvh = Self {
            bounds,
            root: Arc::new(Mutex::new(BVHNode::solid())),
            max_depth,
            destructions: tx,
            destructions_finished: Arc::new(Mutex::new(false)),
            last_optimize: Arc::new(Mutex::new(std::time::Instant::now())),
            updated: Arc::new(Mutex::new(false)),
            optimizations: Arc::new(Mutex::new(max_depth as u32 + 1)),
        };
        bvh.watch_destructions(rx);
        bvh
    }

    pub fn borrow_root(&self) -> MutexGuard<'_, BVHNode> {
        match self.root.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("Mutex poisoned: {poisoned:?}");
                std::process::exit(1);
            }
        }
    }

    pub fn borrow_root_mut(&mut self) -> MutexGuard<'_, BVHNode> {
        match self.root.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("Mutex poisoned: {poisoned:?}");
                std::process::exit(1);
            }
        }
    }

    pub fn draw(&self) {
        self.borrow_root().draw(self.bounds, 0, self.max_depth);
    }

    pub fn get_nodes(&self) -> Vec<(BVHNode, AABB)> {
        let mut nodes = Vec::new();
        self.borrow_root()
            .get_nodes(&self.bounds, 0, self.max_depth, &mut nodes);
        nodes
    }

    pub fn get_nearby_nodes(&self, location: Vec2, radius: f32) -> Vec<(BVHNode, AABB)> {
        let mut nodes = Vec::new();
        self.borrow_root().get_nearby_nodes(
            &self.bounds,
            location,
            radius,
            0,
            self.max_depth,
            &mut nodes,
        );
        nodes
    }

    pub fn cut_circle(&mut self, location: Vec2, radius: f32) {
        if let Err(e) = self
            .destructions
            .send(DestructionType::Circle(location, radius))
        {
            eprintln!("Failed to send destruction task: {e}");
        }
    }

    pub fn cut_point(&mut self, location: Vec2) {
        if let Err(e) = self.destructions.send(DestructionType::Point(location)) {
            eprintln!("Failed to send destruction task: {e}");
        }
    }

    pub fn find_intersects_circle(&mut self, location: Vec2, radius: f32) -> Vec<(BVHNode, AABB)> {
        let bounds = self.bounds;
        let max_depth = self.max_depth;
        let mut nodes = Vec::new();
        self.borrow_root_mut()
            .find_intersects_circle(bounds, location, radius, 0, max_depth, &mut nodes);
        nodes
    }

    pub fn is_updated(&self) -> bool {
        match self.updated.lock() {
            Ok(guard) => *guard,
            Err(poisoned) => {
                eprintln!("Mutex poisoned: {poisoned:?}");
                false
            }
        }
    }

    pub fn set_updated(&self, updated: bool) {
        match self.updated.lock() {
            Ok(mut guard) => *guard = updated,
            Err(poisoned) => {
                eprintln!("Mutex poisoned: {poisoned:?}");
            }
        }
    }

    pub fn watch_destructions(&self, rx: Receiver<DestructionType>) {
        let root = Arc::clone(&self.root);
        let finished = Arc::clone(&self.destructions_finished);
        let bounds = self.bounds;
        let max_depth = self.max_depth;
        let last_optimize = Arc::clone(&self.last_optimize);
        let optimizations = Arc::clone(&self.optimizations);
        let self_updated = Arc::clone(&self.updated);

        thread::spawn(move || {
            loop {
                crossbeam::select! {
                    recv(rx) -> msg => {
                        match finished.lock() {
                            Ok(mut guard) => {
                                *guard = false;
                            }
                            Err(poisoned) => {
                                eprintln!("Mutex poisoned: {poisoned:?}");
                            }
                        }

                        match msg {
                            Ok(task) => {
                                let mut root_node = root.lock().handle("Failed to lock root mutex");
                                match task {
                                    DestructionType::Circle(pos, radius) => {
                                        let _ = root_node.cut_circle(bounds, pos, radius, 0, max_depth);
                                    }
                                    DestructionType::Point(pos) => {
                                        root_node.cut_point(bounds, pos, 0, max_depth);
                                    }
                                }
                                *optimizations.lock().handle("Failed to lock optimizations mutex") = max_depth as u32 + 1;
                            }
                            Err(_) => {
                                break;
                            }
                        }
                    }
                    default(std::time::Duration::from_millis(10)) => {
                        match finished.lock() {
                            Ok(mut guard) => {
                                *guard = true;
                            }
                            Err(poisoned) => {
                                eprintln!("Mutex poisoned: {poisoned:?}");
                            }
                        }

                        let mut optimizations = optimizations.lock().handle("Failed to lock optimizations mutex");
                        if *optimizations > 0 {
                            let mut last_optimize = last_optimize.lock().handle("Failed to lock last_optimize mutex");
                            if last_optimize.elapsed() > std::time::Duration::from_millis(250) {
                                let mut updated = false;
                                root.lock().handle("Failed to lock root mutex").optimize(bounds, 0, max_depth, &mut updated);
                                if updated {
                                    *self_updated.lock().handle("Failed to lock self_updated mutex") = true;
                                }
                                *optimizations -= 1;
                                *last_optimize = std::time::Instant::now();
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn are_destructions_finished(&self) -> bool {
        match self.destructions_finished.lock() {
            Ok(guard) => *guard,
            Err(poisoned) => {
                eprintln!("Mutex poisoned: {poisoned:?}");
                false
            }
        }
    }

    pub fn wait_till_finished(&self) {
        loop {
            if self.are_destructions_finished() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::BVHNodeType;

    #[test]
    fn new() {
        let bvh = BVH::new(800, 600, 5);
        assert_eq!(bvh.bounds.min, vec2(0.0, 0.0));
        assert_eq!(bvh.bounds.max, vec2(800.0, 600.0));
        assert!(matches!(bvh.borrow_root().ty, BVHNodeType::Solid));
        assert_eq!(bvh.max_depth, 5);
    }
}
