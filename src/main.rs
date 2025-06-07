mod app;

use anyhow::{Context, Result};

fn run() -> Result<()> {
    app::create_and_run().context("Failed to create and run the application")?;

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => println!("Application exited successfully."),
        Err(e) => eprintln!("Application encountered an error: {:?}", e),
    }
}
