mod manager;
mod no_scene;
mod scene;

pub use manager::*;
pub use no_scene::*;
pub use scene::*;

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::GameData;

    use anyhow::Result;
    use std::sync::{Arc, Mutex};

    struct TestScene1;

    impl Scene for TestScene1 {
        fn create(_data: Option<Arc<Mutex<GameData>>>) -> Result<Self> {
            Ok(Self)
        }

        fn name(&self) -> &str {
            "test_scene_1"
        }

        fn should_transfer(&self) -> Option<String> {
            Some("test_scene_2".to_string())
        }
    }

    struct TestScene2;

    impl Scene for TestScene2 {
        fn create(_data: Option<Arc<Mutex<GameData>>>) -> Result<Self> {
            Ok(Self)
        }

        fn name(&self) -> &str {
            "test_scene_2"
        }
    }

    #[test]
    fn create() -> Result<()> {
        let scene = TestScene1::create(None)?;
        assert_eq!(scene.name(), "test_scene_1",);
        Ok(())
    }

    #[test]
    fn transfer() -> Result<()> {
        let mut manager = SceneManager::new(None)?;
        manager.add_scene(TestScene1::create(None)?)?;
        manager.add_scene(TestScene2::create(None)?)?;
        manager.transfer_to("test_scene_1".to_string())?;
        manager.update()?;
        assert_eq!(manager.current_scene()?, "test_scene_2");
        Ok(())
    }

    #[test]
    fn transfer_fail() -> Result<()> {
        let mut manager = SceneManager::new(None)?;
        manager.add_scene(TestScene1::create(None)?)?;
        manager.add_scene(TestScene2::create(None)?)?;
        manager.transfer_to("test_scene_1".to_string())?;
        manager.update()?;
        manager.transfer_to("non_existent_scene".to_string())?;
        assert_eq!(manager.current_scene()?, "no_scene");
        Ok(())
    }
}
