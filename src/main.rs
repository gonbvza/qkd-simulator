use std::env;
use std::sync::Arc;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

use crate::cli::cli::run_cli;
use crate::event_loop::EventLoop;

mod api;
mod cli;
mod database;
mod error;
mod event_loop;
mod events;
mod models;
mod nodes;
mod registry;
mod schema;
mod settings;
mod tests;
mod utility;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_loop() {
    loop {
        let loop_pair = Arc::clone(&*EventLoop::instance());
        let (event_loop, _) = &*loop_pair;
        let event = &event_loop
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .bin_heap
            .extract_min();

        match event {
            Some(event) => {
                let _ = &event_loop
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .exec_event(event.clone());
            }
            None => {
                let pair = EventLoop::instance().clone();
                let (event_loop, cvar) = &*pair;

                let guard = event_loop.lock().unwrap();
                println!("obtained");
                let _unused = cvar.wait(guard).unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let loop_pair = Arc::clone(&*EventLoop::instance());
    let (event_loop, _) = &*loop_pair;
    &event_loop
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .instantiate_functions();

    // Spawn CLI (async)
    tokio::spawn(async move {
        match run_cli().await {
            Ok(_) => println!("Loop closed succesfully"),
            Err(e) => println!("{}", e),
        }
    });

    run_loop();
}
