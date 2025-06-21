use rhai::{Engine, Scope, AST};
use std::path::PathBuf;
use tracing::{debug, error, trace};

pub struct Script {
    engine: Engine,
    ast: Option<AST>,
    scope: Scope<'static>,
    path: PathBuf,
    this: rhai::Map,
}

impl Script {
    pub fn new(script_path: impl Into<PathBuf>) -> Self {
        let engine = Engine::new();
        let path = script_path.into();
        let ast = None;
        let scope = Scope::new();
        let this = rhai::Map::new();

        Self {
            engine,
            ast,
            scope,
            path,
            this,
        }
    }

    pub fn try_reload(&mut self) {
        trace!("Attempting to reload script: {}", self.path.display());
        match std::fs::read_to_string(&self.path) {
            Ok(source) => match self.engine.compile(&source) {
                Ok(ast) => {
                    debug!("Reloading script: {}", self.path.display());
                    self.ast = Some(ast);
                    self.init();
                }
                Err(e) => error!("Error compiling script {}: {}", self.path.display(), e),
            },
            Err(e) => error!("Error reading script {}: {}", self.path.display(), e),
        }
    }

    pub fn call(&mut self, fn_name: &str) {
        if let Some(ast) = &self.ast {
            if let Err(e) = self.engine.call_fn::<()>(&mut self.scope, ast, fn_name, ()) {
                error!(
                    "Error calling function `{}` in script {}: {}",
                    fn_name,
                    self.path.display(),
                    e
                );
            }
        }
    }

    pub fn init(&mut self) {
        self.call("init");
    }

    pub fn update(&mut self) {
        self.call("update");
    }

    pub fn render(&mut self) {
        self.call("render");
    }

    pub fn ui(&mut self) {
        self.call("ui");
    }
}
