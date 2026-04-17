use std::env;
use std::sync::{Arc, MutexGuard};

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

use crate::cli::cli::run_cli;
use crate::core::event_loop::EventLoop;
use crate::core::registry::Registry;

mod api;
mod cli;
mod core;
mod database;
mod error;
mod events;
mod models;
mod nodes;
mod schema;
mod tests;
mod utility;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_loop(mut registry: Registry) {
    loop {
        let event = {
            let loop_pair = Arc::clone(&*EventLoop::instance());
            let (event_loop, _) = &*loop_pair;

            let mut guard = event_loop.lock().unwrap();
            let event = guard.bin_heap.extract_min();
            if let Some(event) = event.clone() {
                guard.set_new_timestamp(&event.timestamp);
            }
            event
        };

        match event {
            Some(event) => {
                let _ = &registry.exec_event(event.to_owned());
            }
            None => {
                let pair = EventLoop::instance().clone();
                let (event_loop, cvar) = &*pair;

                let guard = event_loop.lock().unwrap();
                let _unused = cvar.wait(guard).unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut registry = Registry::new();
    registry.instantiate_functions();

    // Spawn CLI (async)
    tokio::spawn(async move {
        match run_cli().await {
            Ok(_) => println!("Loop closed succesfully"),
            Err(e) => println!("{}", e),
        }
    });

    run_loop(registry);
}
