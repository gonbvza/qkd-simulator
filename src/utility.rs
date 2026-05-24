use std::io;

use crate::{
    error::PairError,
    models::entangled_pair::{NewEntangledPair, Side},
};

pub fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    return buffer.trim().to_string();
}

pub fn is_first(entangled_pair: &mut NewEntangledPair, side: Side) -> Result<bool, PairError> {
    match side {
        Side::Source => Ok(entangled_pair.dst_measurement.is_none()),
        Side::Destination => Ok(entangled_pair.src_measurement.is_none()),
    }
}
