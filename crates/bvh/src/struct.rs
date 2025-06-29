use anyhow::{Context, Result};
use macroquad::prelude::*;

use super::{AABB, BVHNode};

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

    pub fn get_nodes(&self) -> Vec<(BVHNode, AABB)> {
        let mut nodes = Vec::new();
        self.root
            .get_nodes(&self.bounds, 0, self.max_depth, &mut nodes);
        nodes
    }

    pub fn get_nearby_nodes(&self, location: Vec2, radius: f32) -> Vec<(BVHNode, AABB)> {
        let mut nodes = Vec::new();
        self.root.get_nearby_nodes(
            &self.bounds,
            location,
            radius,
            0,
            self.max_depth,
            &mut nodes,
        );
        nodes
    }

    pub fn cut_circle(&mut self, location: Vec2, radius: f32) -> Result<()> {
        Self::cut_circle_node(
            &mut self.root,
            self.bounds,
            location,
            radius,
            0,
            self.max_depth,
        )
    }

    fn cut_circle_node(
        node: &mut BVHNode,
        node_bounds: AABB,
        location: Vec2,
        radius: f32,
        depth: usize,
        max_depth: usize,
    ) -> Result<()> {
        match node {
            BVHNode::Empty => {}
            BVHNode::Solid => {
                if depth >= max_depth {
                    *node = BVHNode::Empty;
                    return Ok(());
                }

                if node_bounds.contains_circle(location, radius) {
                    *node = BVHNode::Empty;
                    return Ok(());
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
                    return Ok(());
                }

                *node = BVHNode::Internal {
                    children: Box::new([
                        BVHNode::Solid,
                        BVHNode::Solid,
                        BVHNode::Solid,
                        BVHNode::Solid,
                    ]),
                };

                let children = node.children_mut().context("Failed to get children")?;
                for (i, cb) in child_bounds.iter().enumerate() {
                    if intersects[i] {
                        Self::cut_circle_node(
                            &mut children[i],
                            *cb,
                            location,
                            radius,
                            depth + 1,
                            max_depth,
                        )
                        .context("Failed to cut circle node")?;
                    }
                }
            }
            BVHNode::Internal { children } => {
                if children.iter().all(|c| matches!(c, BVHNode::Empty)) {
                    *node = BVHNode::Empty;
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
                        Self::cut_circle_node(
                            &mut children[i],
                            *cb,
                            location,
                            radius,
                            depth + 1,
                            max_depth,
                        )
                        .context("Failed to cut circle node")?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn cut_point(&mut self, location: Vec2) {
        Self::cut_point_node(&mut self.root, self.bounds, location, 0, self.max_depth);
    }

    fn cut_point_node(
        node: &mut BVHNode,
        node_bounds: AABB,
        location: Vec2,
        depth: usize,
        max_depth: usize,
    ) {
        match node {
            BVHNode::Empty => {}
            BVHNode::Solid => {
                if depth >= max_depth || !node_bounds.contains_point(location) {
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
            }
            BVHNode::Internal { children } => {
                if children.iter().all(|c| matches!(c, BVHNode::Empty)) {
                    *node = BVHNode::Empty;
                    return;
                }
                let child_bounds = node_bounds.subdivide();
                for (i, cb) in child_bounds.iter().enumerate() {
                    if cb.contains_point(location) {
                        Self::cut_point_node(&mut children[i], *cb, location, depth + 1, max_depth);
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
    fn new() {
        let bvh = BVH::new(800, 600, 5);
        assert_eq!(bvh.bounds.min, vec2(0.0, 0.0));
        assert_eq!(bvh.bounds.max, vec2(800.0, 600.0));
        assert!(matches!(bvh.root, BVHNode::Solid));
        assert_eq!(bvh.max_depth, 5);
    }

    #[test]
    fn get_nearby_nodes() -> Result<()> {
        let mut bvh = BVH::new(800, 600, 5);
        bvh.cut_circle(vec2(0.0, 0.0), 50.0)?;
        let nodes = bvh.get_nearby_nodes(vec2(0.0, 0.0), 200.0);
        assert_eq!(nodes.len(), 11);
        Ok(())
    }

    #[test]
    fn cut_circle() -> Result<()> {
        let mut bvh = BVH::new(800, 600, 5);
        bvh.cut_circle(vec2(20.0, 15.0), 50.0)?;
        let nodes = bvh.get_nearby_nodes(vec2(0.0, 0.0), 1000.0);
        assert_eq!(nodes.len(), 14);
        Ok(())
    }

    #[test]
    fn cut_point() {
        let mut bvh = BVH::new(800, 600, 5);
        bvh.cut_point(vec2(400.0, 300.0));
        let nodes = bvh.get_nearby_nodes(vec2(400.0, 300.0), 200.0);
        assert_eq!(nodes.len(), 4);
    }
}
