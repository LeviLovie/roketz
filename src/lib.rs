mod config;
mod world;

pub use config::*;
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

pub async fn handle_result_async_closure<F, V>(f: F) -> V
where
    F: std::future::Future<Output = anyhow::Result<V>>,
{
    handle_result(f.await)
}
