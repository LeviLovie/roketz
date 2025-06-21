use tracing::debug;

use super::Script;

pub struct Engine {
    directory: std::path::PathBuf,
    pub scripts: Vec<Script>,
}

impl Engine {
    pub fn new(directory: impl Into<std::path::PathBuf>) -> Self {
        Self {
            directory: directory.into(),
            scripts: Vec::new(),
        }
    }

    pub fn add_script(&mut self, script: Script) {
        self.scripts.push(script);
    }

    pub fn load(&mut self, script_path: impl Into<String>) {
        let script_path = script_path.into();
        let path = match script_path.ends_with(".rhai") {
            true => self.directory.join(script_path),
            false => self.directory.join(format!("{}.rhai", script_path)),
        };
        let path = std::path::PathBuf::from(path);

        debug!(path = ?path.display(), "Loading script");
        self.add_script(Script::new(path));
    }

    pub fn reload(&mut self) {
        for script in &mut self.scripts {
            script.try_reload();
        }
    }

    pub fn init(&mut self) {
        for script in &mut self.scripts {
            script.init();
        }
    }

    pub fn update(&mut self) {
        for script in &mut self.scripts {
            script.update();
        }
    }

    pub fn render(&mut self) {
        for script in &mut self.scripts {
            script.render();
        }
    }

    pub fn ui(&mut self) {
        for script in &mut self.scripts {
            script.ui();
        }
    }
}
