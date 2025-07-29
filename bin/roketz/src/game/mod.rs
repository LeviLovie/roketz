mod data;
mod manager;
mod scenes;

pub use data::*;
pub use scenes::*;

use helpers::error::HandleError;

pub async fn run() {
    manager::start()
        .await
        .handle("Failed to start the game manager");
}
