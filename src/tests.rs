#[cfg(test)]
mod config_tests {
    use super::super::config::Config;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.window.width, 800);
        assert_eq!(config.window.height, 600);
        assert_eq!(config.graphics.scale, 4);
        assert_eq!(config.physics.bvh_depth, 8);
        assert_eq!(config.physics.max_crash_velocity, 50.0);
        assert_eq!(config.assets, "assets.bin");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = ron::ser::to_string(&config).unwrap();
        let deserialized: Config = ron::from_str(&serialized).unwrap();
        assert_eq!(config.window.width, deserialized.window.width);
        assert_eq!(config.window.height, deserialized.window.height);
    }
}

#[cfg(test)]
mod bvh_tests {
    use super::super::bvh::{AABB, BVH, BVHNode};

    #[test]
    fn test_aabb_creation() {
        let aabb = AABB::new(0.0, 0.0, 10.0, 10.0);
        assert_eq!(aabb.min_x, 0.0);
        assert_eq!(aabb.min_y, 0.0);
        assert_eq!(aabb.max_x, 10.0);
        assert_eq!(aabb.max_y, 10.0);
    }

    #[test]
    fn test_aabb_intersection() {
        let aabb1 = AABB::new(0.0, 0.0, 10.0, 10.0);
        let aabb2 = AABB::new(5.0, 5.0, 15.0, 15.0);
        let aabb3 = AABB::new(20.0, 20.0, 30.0, 30.0);

        assert!(aabb1.intersects(&aabb2));
        assert!(!aabb1.intersects(&aabb3));
    }

    #[test]
    fn test_bvh_node_creation() {
        let node = BVHNode::new_leaf(1, AABB::new(0.0, 0.0, 10.0, 10.0));
        assert!(node.is_leaf());
        assert_eq!(node.entity_id(), Some(1));
    }
}

#[cfg(test)]
mod result_tests {
    use super::super::result::handle_result;
    use anyhow::Result;

    #[test]
    fn test_success_result() {
        let result: Result<()> = Ok(());
        // This should not panic
        handle_result(result);
    }

    #[test]
    fn test_error_result() {
        let result: Result<()> = Err(anyhow::anyhow!("Test error"));
        // This should log the error but not panic
        handle_result(result);
    }
}
