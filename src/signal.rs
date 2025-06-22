use anyhow::Result;
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use std::thread;
use tracing::{debug, info, warn};

pub fn install_signal_handler() -> Result<()> {
    let mut signals = Signals::new([SIGINT, SIGTERM, SIGQUIT])?;
    thread::spawn(move || {
        signals.forever().for_each(|signal| {
            match signal {
                SIGINT => info!("Received SIGINT (Ctrl+C)"),
                SIGTERM => warn!("Received SIGTERM (kill)"),
                SIGQUIT => info!("Received SIGQUIT (Ctrl+\\)"),
                _ => info!("Received unknown signal: {}", signal),
            }

            debug!("Exiting...");
            std::process::exit(0);
        });
    });
    Ok(())
}
