mod scenes;
mod manager;
mod data;

pub use scenes::*;
pub use data::*;

pub async fn run() {
    crate::result::handle_result(manager::start().await);
}
