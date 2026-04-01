use std::collections::HashMap;
use std::env;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

use crate::cli::run_cli;
use crate::event_loop::EventLoop;
use crate::models::args::EventArgs;

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

fn test_func(args: &HashMap<String, EventArgs>) {
    let qubit = match args.get("qubit") {
        Some(qubit) => Some(qubit),
        None => None,
    };

    if let EventArgs::QubitRef(qubit_ref) = qubit.unwrap() {
        println!("Id is {}", qubit_ref.entangled_pair_id)
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() {
    let mut event_loop = EventLoop::instance()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    event_loop.instantiate_functions();

    // Spawn CLI (async)
    tokio::spawn(async {
        match run_cli().await {
            Ok(_) => println!("Loop closed succesfully"),
            Err(e) => println!("{}", e),
        }
    });

    // Spawn your blocking event loop
    event_loop.run_loop();
}
