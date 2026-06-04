use std::sync::mpsc::{channel, Receiver, Sender};

use crate::cli::runner::run_cli;
use crate::core::event_loop::{EventLoop, EventLoopHandler};
use crate::core::registry::Registry;
use crate::error::Error;
use crate::models::event::Event;
use crate::ui::main::start_server;

pub use utility::establish_connection;

pub mod api;
pub mod cli;
pub mod core;
pub mod database;
pub mod error;
pub mod events;
pub mod models;
pub mod schema;
#[cfg(test)]
pub mod tests;
pub mod ui;
pub mod utility;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (tx, rx): (Sender<Event>, Receiver<Event>) = channel();
    let handler: EventLoopHandler = EventLoopHandler::new(tx.clone());
    let sim_handler: EventLoopHandler = EventLoopHandler::new(tx.clone());

    // Spawn web application (async)
    tokio::spawn(async {
        start_server().await;
    });

    // Spawn CLI (async)
    tokio::spawn(async move {
        match run_cli(handler).await {
            Ok(_) => println!("Loop closed succesfully"),
            Err(e) => println!("{}", e),
        }
    });

    // Create event loop and registry
    let registry = Registry::new();

    // Create event loop that run events
    let mut event_loop = EventLoop::new();
    event_loop.run_loop(registry, sim_handler, rx)?;

    Ok(())
}
