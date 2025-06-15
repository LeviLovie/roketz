use macroquad::prelude::*;

use super::{BVHNode, AABB};

pub struct BVH {
    bounds: AABB,
    root: BVHNode,
    max_depth: usize,
}

impl BVH {
    pub fn new(width: u32, height: u32, max_depth: usize) -> Self {
        let bounds = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(width as f32, height as f32),
        };

        Self {
            bounds,
            root: BVHNode::Solid,
            max_depth,
        }
    }

    pub fn draw(&self) {
        self.root.draw(self.bounds, 0, self.max_depth);
    }

    pub fn cut_circle(&mut self, location: Vec2, radius: f32) {
        Self::cut_circle_node(
            &mut self.root,
            self.bounds,
            location,
            radius,
            0,
            self.max_depth,
        );
    }

    fn cut_circle_node(
        node: &mut BVHNode,
        node_bounds: AABB,
        location: Vec2,
        radius: f32,
        depth: usize,
        max_depth: usize,
    ) {
        match node {
            BVHNode::Empty => {}
            BVHNode::Solid => {
                if depth >= max_depth {
                    *node = BVHNode::Empty;
                    return;
                }

                if node_bounds.inside_circle(location, radius) {
                    *node = BVHNode::Empty;
                    return;
                }

                *node = BVHNode::Internal {
                    children: Box::new([
                        BVHNode::Solid,
                        BVHNode::Solid,
                        BVHNode::Solid,
                        BVHNode::Solid,
                    ]),
                };

                let child_bounds = node_bounds.subdivide();
                let mut intersects = [false; 4];

                for (i, cb) in child_bounds.iter().enumerate() {
                    if cb.intersects_circle(location, radius) {
                        intersects[i] = true;
                    }
                }

                if !intersects.iter().any(|&b| b) {
                    return;
                }

                *node = BVHNode::Internal {
                    children: Box::new([
                        BVHNode::Solid,
                        BVHNode::Solid,
                        BVHNode::Solid,
                        BVHNode::Solid,
                    ]),
                };

                let children = node.children_mut().unwrap();
                for (i, cb) in child_bounds.iter().enumerate() {
                    if intersects[i] {
                        Self::cut_circle_node(
                            &mut children[i],
                            *cb,
                            location,
                            radius,
                            depth + 1,
                            max_depth,
                        );
                    }
                }
            }
            BVHNode::Internal { children } => {
                if children.iter().all(|c| matches!(c, BVHNode::Empty)) {
                    *node = BVHNode::Empty;
                    return;
                }

                let child_bounds = node_bounds.subdivide();
                let mut intersections = [false; 4];

                for (i, cb) in child_bounds.iter().enumerate() {
                    if cb.inside_circle(location, radius) {
                        intersections[i] = true;
                    } else if cb.intersects_circle(location, radius) {
                        intersections[i] = true;
                    }
                }

                if !intersections.iter().any(|&b| b) {
                    return;
                }

                for (i, cb) in child_bounds.iter().enumerate() {
                    if intersections[i] {
                        Self::cut_circle_node(
                            &mut children[i],
                            *cb,
                            location,
                            radius,
                            depth + 1,
                            max_depth,
                        );
                    }
                }
            }
        }
    }
}
