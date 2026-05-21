use std::io;

use crate::{
    error::PairError,
    models::{entangled_pair::NewEntangledPair, qubit_ref::QubitRefSide},
};

pub fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    return buffer.trim().to_string();
}

pub fn is_first(
    entangled_pair: &mut NewEntangledPair,
    side: QubitRefSide,
) -> Result<bool, PairError> {
    match side {
        QubitRefSide::Source => Ok(entangled_pair.dst_measurement.is_none()),
        QubitRefSide::Destination => Ok(entangled_pair.src_measurement.is_none()),
    }
}
