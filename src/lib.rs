mod config;
mod game;
mod world;

pub use config::*;
pub use game::*;
pub use world::*;

pub fn handle_result<V>(result: anyhow::Result<V>) -> V {
    result.unwrap_or_else(|e| {
        eprintln!("An error occurred: {:?}", e);
        std::process::exit(1);
    })
}

pub fn handle_result_closure<F, V>(f: F) -> V
where
    F: FnOnce() -> anyhow::Result<V>,
{
    handle_result(f())
}
