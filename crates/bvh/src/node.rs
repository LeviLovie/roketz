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
        match self {
            BVHNode::Empty => {}
            BVHNode::Solid => {
                nodes.push((self.clone(), *bounds));
            }
            BVHNode::Internal { children } => {
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
        match self {
            BVHNode::Empty => {}
            BVHNode::Solid => {
                if bounds.intersects_circle(location, radius) {
                    nodes.push((self.clone(), *bounds));
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn children() {
        let mut node = BVHNode::Internal {
            children: Box::new([
                BVHNode::Empty,
                BVHNode::Solid,
                BVHNode::Empty,
                BVHNode::Empty,
            ]),
        };
        assert!(node.children().is_some());
        assert!(node.children_mut().is_some());
    }

    #[test]
    fn get_nearby_nodes() {
        let node = BVHNode::Internal {
            children: Box::new([
                BVHNode::Solid,
                BVHNode::Empty,
                BVHNode::Internal {
                    children: Box::new([
                        BVHNode::Empty,
                        BVHNode::Solid,
                        BVHNode::Empty,
                        BVHNode::Empty,
                    ]),
                },
                BVHNode::Empty,
            ]),
        };
        let bounds = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(10.0, 10.0),
        };
        let mut nodes = Vec::new();
        node.get_nearby_nodes(&bounds, vec2(5.0, 5.0), 1.0, 0, 2, &mut nodes);
        assert_eq!(nodes.len(), 2);
    }
}
