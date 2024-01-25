use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use tokio::signal::windows::ctrl_c;
use crate::services::runner::Runner;

mod traits;
mod protocols;
mod enums;
mod operations;
mod controllers;
mod utils;
mod services;
mod models;
mod filters;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let sniffing_activated = Arc::new(AtomicBool::new(false));

    let mut threads = Vec::new();
    let mut signal = ctrl_c()?;

    threads.push(thread::spawn({
        let sniffing_activated = sniffing_activated.clone();
        let running = running.clone();

        move || {
            let mut runner = Runner::new(
                running,
                sniffing_activated
            );

            runner.run();
        }
    }));


    while running.load(Ordering::Relaxed) {
        while let Some(()) = signal.recv().await {
            sniffing_activated.store(false, Ordering::Relaxed);
        }
    }

    Ok(())
}
