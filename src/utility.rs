use std::env;
use std::io;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

use crate::{
    error::PairError,
    models::entangled_pair::{NewEntangledPair, Side},
};

/// Opens a PostgreSQL connection using `DATABASE_URL` from the environment.
///
/// This is the shared entry point for database access across the binary,
/// API handlers, models, and tests.
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_string()
}

pub fn is_first(entangled_pair: &mut NewEntangledPair, side: Side) -> Result<bool, PairError> {
    match side {
        Side::Source => Ok(entangled_pair.dst_measurement.is_none()),
        Side::Destination => Ok(entangled_pair.src_measurement.is_none()),
    }
}
