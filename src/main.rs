use std::env;
use std::sync::mpsc::{Receiver, Sender, channel};

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

use crate::cli::runner::run_cli;
use crate::core::event_loop::{EventLoop, EventLoopHandler};
use crate::core::registry::Registry;
use crate::error::Error;
use crate::models::event::Event;

mod api;
mod cli;
mod core;
mod database;
mod error;
mod events;
mod models;
mod schema;
#[cfg(test)]
mod tests;
mod utility;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_loop(
    mut registry: Registry,
    handle: EventLoopHandler,
    rx: Receiver<Event>,
) -> Result<(), Error> {
    let mut event_loop = EventLoop::new();

    loop {
        while let Ok(event) = rx.try_recv() {
            event_loop.push_event(event, event_loop.get_current_time());
        }

        let Some(scheduled_event) = event_loop.pop_next_event() else {
            match rx.recv() {
                Ok(event) => {
                    event_loop.push_event(event, event_loop.get_current_time());
                    continue;
                }
                Err(_) => return Ok(()),
            }
        };

        event_loop.set_new_timestamp(&scheduled_event.timestamp);
        let event_name = scheduled_event.event.name.clone();
        let timestamp = scheduled_event.timestamp;

        if let Err(e) = registry.exec_event(scheduled_event, &handle) {
            eprintln!(
                "Event {:?} at t={} failed; continuing loop: {}",
                event_name, timestamp, e
            );
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (tx, rx): (Sender<Event>, Receiver<Event>) = channel();
    let handler: EventLoopHandler = EventLoopHandler::new(tx.clone());
    let sim_handler: EventLoopHandler = EventLoopHandler::new(tx.clone());

    // Spawn CLI (async)
    tokio::spawn(async move {
        match run_cli(handler).await {
            Ok(_) => println!("Loop closed succesfully"),
            Err(e) => println!("{}", e),
        }
    });

    // Create event loop and registry
    let registry = Registry::new();

    run_loop(registry, sim_handler, rx)?;
    Ok(())
}
