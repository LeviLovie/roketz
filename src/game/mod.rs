mod data;
mod manager;
mod scenes;

pub use data::*;
pub use scenes::*;

pub async fn run() {
    crate::result::handle_result(manager::start().await);
}
