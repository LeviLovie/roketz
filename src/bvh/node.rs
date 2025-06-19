use macroquad::prelude::*;

use super::AABB;

#[derive(Debug, Clone)]
pub enum BVHNode {
    Solid,
    Empty,
    Internal { children: Box<[BVHNode; 4]> },
}

impl BVHNode {
    pub fn children(&self) -> Option<&[BVHNode; 4]> {
        if let BVHNode::Internal { children } = self {
            Some(children)
        } else {
            None
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut [BVHNode; 4]> {
        if let BVHNode::Internal { children } = self {
            Some(children)
        } else {
            None
        }
    }

    pub fn get_nearby_nodes(
        &self,
        bounds: &AABB,
        location: Vec2,
        radius: f32,
        depth: usize,
        max_depth: usize,
        nodes: &mut Vec<BVHNode>,
    ) {
        if depth > max_depth {
            return;
        }
        match self {
            BVHNode::Empty => {}
            BVHNode::Solid => {
                if bounds.contains_circle(location, radius) {
                    nodes.push(self.clone());
                }
            }
            BVHNode::Internal { children } => {
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

        match self {
            BVHNode::Empty => {
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
            BVHNode::Solid => {
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
                    Color::from_rgba(0, 255, 0, 50),
                );
            }
            BVHNode::Internal { children } => {
                let child_bounds = bounds.subdivide();
                for (i, child) in children.iter().enumerate() {
                    child.draw(child_bounds[i], depth + 1, max_depth);
                }
            }
        }
    }
}
