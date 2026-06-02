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

/// Function to split qubit vector into smaller vectors
pub fn split_vector(vector: Vec<u8>, blocks: usize) -> Vec<Vec<u8>> {
    let each_len = vector.len() / blocks;
    let mut src_blocks: Vec<Vec<u8>> = vec![Vec::with_capacity(each_len); blocks];
    for (i, d) in vector.iter().copied().enumerate() {
        let idx = i / each_len;

        src_blocks[idx].push(d);
    }
    src_blocks
}

/// Function to calculate parity of a block
pub fn parity(vector: &Vec<u8>) -> u8 {
    let parity: u16 = vector.iter().map(|&x| x as u16).sum();
    (parity % 2) as u8
}

/// Function to convert bit key into a byte key
pub fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    bits.chunks(8)
        .map(|chunk| chunk.iter().fold(0u8, |acc, &b| (acc << 1) | (b & 1)))
        .collect()
}
