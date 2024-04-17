mod cli;
mod types;

use crate::cli::PhotoSyncCli;
use clap::Parser;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().expect("Cannot init logger.");
    match PhotoSyncCli::parse().sync_photos() {
        Ok(missing) => {
            missing.iter().for_each(|(day, missing)| {
                    if missing.is_empty() {
                        log::info!("For day: '{day}' no difference have been found.")
                    } else {
                        missing.iter().for_each(|(name, missing_photos)| {
                            log::info!(
                                "For day: '{}', you missing: [{:?}] - we can find it in '{}' collection.",
                                day, missing_photos, name,
                            )
                        })
                    }
                });
        }
        Err(e) => log::error!("Something unexpected happen - {}", e),
    }
}
