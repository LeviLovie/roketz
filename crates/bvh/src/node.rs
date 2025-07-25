use anyhow::{Context, Result};
use macroquad::prelude::*;
use std::sync::{Arc, LazyLock, Mutex, atomic::AtomicU32};

static LAST_NODE_ID: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(0));

use super::AABB;

#[derive(Debug, Clone, Default)]
pub enum BVHNodeType {
    Solid,
    #[default]
    Empty,
    Internal {
        children: Box<[BVHNode; 4]>,
    },
}

#[derive(Debug, Clone)]
pub struct BVHNode {
    pub ty: BVHNodeType,
    pub id: u32,
    pub updated: Arc<Mutex<bool>>,
}

impl BVHNode {
    fn new_id() -> u32 {
        LAST_NODE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        LAST_NODE_ID.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn empty() -> Self {
        BVHNode {
            ty: BVHNodeType::Empty,
            id: Self::new_id(),
            updated: Arc::new(Mutex::new(true)),
        }
    }

    pub fn solid() -> Self {
        BVHNode {
            ty: BVHNodeType::Solid,
            id: Self::new_id(),
            updated: Arc::new(Mutex::new(true)),
        }
    }

    pub fn internal(children: Box<[BVHNode; 4]>) -> Self {
        BVHNode {
            ty: BVHNodeType::Internal { children },
            id: Self::new_id(),
            updated: Arc::new(Mutex::new(true)),
        }
    }

    pub fn children(&self) -> Option<&[BVHNode; 4]> {
        if let BVHNodeType::Internal { children } = &self.ty {
            Some(children)
        } else {
            None
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut [BVHNode; 4]> {
        if let BVHNodeType::Internal { children } = &mut self.ty {
            Some(children)
        } else {
            None
        }
    }

    pub fn set_updated(&self, updated: bool) {
        *self.updated.lock().unwrap() = updated;
    }

    pub fn is_updated(&self) -> bool {
        *self.updated.lock().unwrap()
    }

    pub fn get_nodes(
        &self,
        bounds: &AABB,
        depth: usize,
        max_depth: usize,
        nodes: &mut Vec<(BVHNode, AABB)>,
    ) {
        if depth > max_depth {
            return;
        }
        match &self.ty {
            BVHNodeType::Empty => {}
            BVHNodeType::Solid => {
                nodes.push((self.clone(), *bounds));
            }
            BVHNodeType::Internal { children } => {
                let child_bounds = bounds.subdivide();
                for (i, child) in children.iter().enumerate() {
                    child.get_nodes(&child_bounds[i], depth + 1, max_depth, nodes);
                }
            }
        }
    }

    pub fn get_nearby_nodes(
        &self,
        bounds: &AABB,
        location: Vec2,
        radius: f32,
        depth: usize,
        max_depth: usize,
        nodes: &mut Vec<(BVHNode, AABB)>,
    ) {
        if depth > max_depth {
            return;
        }
        match &self.ty {
            BVHNodeType::Empty => {}
            BVHNodeType::Solid => {
                if bounds.intersects_circle(location, radius) {
                    nodes.push((self.clone(), *bounds));
                }
            }
            BVHNodeType::Internal { children } => {
                let child_bounds = bounds.subdivide();
                for (i, child) in children.iter().enumerate() {
                    if child_bounds[i].intersects_circle(location, radius) {
                        child.get_nearby_nodes(
                            &child_bounds[i],
                            location,
                            radius,
                            depth + 1,
                            max_depth,
                            nodes,
                        );
                    }
                }
            }
        }
    }

    pub fn draw(&self, bounds: AABB, depth: usize, max_depth: usize) {
        if depth > max_depth {
            return;
        }

        match &self.ty {
            BVHNodeType::Empty => {
                draw_rectangle_lines(
                    bounds.min.x,
                    bounds.min.y,
                    bounds.max.x - bounds.min.x,
                    bounds.max.y - bounds.min.y,
                    0.2,
                    RED,
                );
                draw_rectangle(
                    bounds.min.x,
                    bounds.min.y,
                    bounds.max.x - bounds.min.x,
                    bounds.max.y - bounds.min.y,
                    Color::from_rgba(255, 0, 0, 50),
                );
            }
            BVHNodeType::Solid => {
                draw_rectangle_lines(
                    bounds.min.x,
                    bounds.min.y,
                    bounds.max.x - bounds.min.x,
                    bounds.max.y - bounds.min.y,
                    0.2,
                    GREEN,
                );
                draw_rectangle(
                    bounds.min.x,
                    bounds.min.y,
                    bounds.max.x - bounds.min.x,
                    bounds.max.y - bounds.min.y,
                    Color::from_rgba(0, 255, 0, 100),
                );
            }
            BVHNodeType::Internal { children } => {
                let child_bounds = bounds.subdivide();
                for (i, child) in children.iter().enumerate() {
                    child.draw(child_bounds[i], depth + 1, max_depth);
                }
            }
        }
    }

    pub fn cut_circle(
        &mut self,
        node_bounds: AABB,
        location: Vec2,
        radius: f32,
        depth: usize,
        max_depth: usize,
    ) -> Result<()> {
        match &mut self.ty {
            BVHNodeType::Empty => {}
            BVHNodeType::Solid => {
                if depth >= max_depth || node_bounds.contains_circle(location, radius) {
                    self.ty = BVHNodeType::Empty;
                    self.set_updated(true);
                    return Ok(());
                }

                let child_bounds = node_bounds.subdivide();
                let mut intersects = [false; 4];

                for (i, cb) in child_bounds.iter().enumerate() {
                    if cb.intersects_circle(location, radius) {
                        intersects[i] = true;
                    }
                }

                if !intersects.iter().any(|&b| b) {
                    return Ok(());
                }

                self.ty = BVHNodeType::Internal {
                    children: Box::new([
                        BVHNode::solid(),
                        BVHNode::solid(),
                        BVHNode::solid(),
                        BVHNode::solid(),
                    ]),
                };
                self.set_updated(true);

                let children = match self.children_mut() {
                    Some(children) => children,
                    None => return Err(anyhow::anyhow!("Failed to get children")),
                };

                for (i, cb) in child_bounds.iter().enumerate() {
                    if intersects[i] {
                        children[i]
                            .cut_circle(*cb, location, radius, depth + 1, max_depth)
                            .context("Failed to cut circle node")?;
                    }
                }
            }
            BVHNodeType::Internal { children } => {
                if children.iter().all(|c| matches!(c.ty, BVHNodeType::Empty)) {
                    self.ty = BVHNodeType::Empty;
                    self.set_updated(true);
                    return Ok(());
                }

                let child_bounds = node_bounds.subdivide();
                let mut intersections = [false; 4];

                for (i, cb) in child_bounds.iter().enumerate() {
                    if cb.contains_circle(location, radius)
                        || cb.intersects_circle(location, radius)
                    {
                        intersections[i] = true;
                    }
                }

                if !intersections.iter().any(|&b| b) {
                    return Ok(());
                }

                for (i, cb) in child_bounds.iter().enumerate() {
                    if intersections[i] {
                        children[i]
                            .cut_circle(*cb, location, radius, depth + 1, max_depth)
                            .context("Failed to cut circle node")?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn cut_point(&mut self, node_bounds: AABB, location: Vec2, depth: usize, max_depth: usize) {
        match &mut self.ty {
            BVHNodeType::Empty => {}
            BVHNodeType::Solid => {
                if depth >= max_depth || !node_bounds.contains_point(location) {
                    self.ty = BVHNodeType::Empty;
                    self.set_updated(true);
                } else {
                    self.ty = BVHNodeType::Internal {
                        children: Box::new([
                            BVHNode::solid(),
                            BVHNode::solid(),
                            BVHNode::solid(),
                            BVHNode::solid(),
                        ]),
                    };
                    self.set_updated(true);
                }
            }
            BVHNodeType::Internal { children } => {
                if children.iter().all(|c| matches!(c.ty, BVHNodeType::Empty)) {
                    self.ty = BVHNodeType::Empty;
                    self.set_updated(true);
                } else {
                    let child_bounds = node_bounds.subdivide();
                    for (i, cb) in child_bounds.iter().enumerate() {
                        if cb.contains_point(location) {
                            children[i].cut_point(*cb, location, depth + 1, max_depth);
                        }
                    }
                }
            }
        }
    }

    pub fn find_intersects_circle(
        &mut self,
        bounds: AABB,
        location: Vec2,
        radius: f32,
        depth: usize,
        max_depth: usize,
        nodes: &mut Vec<(BVHNode, AABB)>,
    ) {
        match &mut self.ty {
            BVHNodeType::Empty => {}
            BVHNodeType::Solid => {
                if depth < max_depth && bounds.intersects_circle(location, radius) {
                    nodes.push((self.clone(), bounds));
                }
            }
            BVHNodeType::Internal { children } => {
                if depth >= max_depth || !bounds.intersects_circle(location, radius) {
                    return;
                }
                for (i, child) in children.iter_mut().enumerate() {
                    let child_bounds = bounds.subdivide()[i];
                    child.find_intersects_circle(
                        child_bounds,
                        location,
                        radius,
                        depth + 1,
                        max_depth,
                        nodes,
                    );
                }
            }
        }
    }

    pub fn optimize(&mut self, bounds: AABB, depth: usize, max_depth: usize, updated: &mut bool) {
        if depth > max_depth {
            return;
        }

        match &mut self.ty {
            BVHNodeType::Empty => {}
            BVHNodeType::Solid => {}
            BVHNodeType::Internal { children } => {
                if children.iter().all(|c| matches!(c.ty, BVHNodeType::Empty)) {
                    self.ty = BVHNodeType::Empty;
                    self.set_updated(true);
                    *updated = true;
                } else if children.iter().all(|c| matches!(c.ty, BVHNodeType::Solid)) {
                    self.ty = BVHNodeType::Solid;
                    self.set_updated(true);
                    *updated = true;
                } else {
                    let child_bounds = bounds.subdivide();
                    for (i, child) in children.iter_mut().enumerate() {
                        child.optimize(child_bounds[i], depth + 1, max_depth, updated);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn children() {
        let mut node = BVHNode::internal(Box::new([
            BVHNode::empty(),
            BVHNode::solid(),
            BVHNode::empty(),
            BVHNode::empty(),
        ]));
        assert!(node.children().is_some());
        assert!(node.children_mut().is_some());
    }

    #[test]
    fn get_nearby_nodes() {
        let node = BVHNode::internal(Box::new([
            BVHNode::solid(),
            BVHNode::empty(),
            BVHNode::internal(Box::new([
                BVHNode::empty(),
                BVHNode::solid(),
                BVHNode::empty(),
                BVHNode::empty(),
            ])),
            BVHNode::empty(),
        ]));
        let bounds = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(10.0, 10.0),
        };
        let mut nodes = Vec::new();
        node.get_nearby_nodes(&bounds, vec2(5.0, 5.0), 1.0, 0, 2, &mut nodes);
        assert_eq!(nodes.len(), 2);
    }
}
