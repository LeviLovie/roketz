mod app;
mod data;
mod logging;
mod scenes;
mod settings;

use utils::prelude::*;

#[main_pretty_error]
fn main() -> Result<()> {
    logging::init();
    logging::debug();

    let data = {
        let settings = settings::Settings::load().context("Loading settings")?;
        let data = data::GameData::new(settings);
        MArc::new(data, "GameData")
    };

    info!("Starting App");
    app::App::new(data.clone())
        .context("Creating a new App")?
        .run()
        .context("Running App")?;

    debug!("App exited, saving settings");
    data.lock()?.settings.save().context("Saving settings")?;

    info!("Exiting");
    Ok(())
}
