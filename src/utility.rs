use std::{collections::HashMap, io};

use crate::{
    database::{entangled_pair::get_pair, nodes::get_node_by_id},
    error::{Error, PairError, SimError},
    establish_connection,
    models::{
        args::EventArgs, entangled_pair::NewEntangledPair, measurement::Measurement,
        qubit_ref::QubitRefSide,
    },
    nodes::node::Node,
};

pub fn verify_args() {
    todo!()
}

pub fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    return buffer.trim().to_string();
}

#[macro_export]
macro_rules! get_node_arg {
    ($args:expr, $key:expr) => {{
        match $args.get($key) {
            Some(EventArgs::Node(node)) => node,
            _ => {
                return Err(Error::Sim(SimError::MissingArgument($key.to_string())));
            }
        }
    }};
}

#[macro_export]
macro_rules! get_string_arg {
    ($args:expr, $key:expr) => {{
        match $args.get($key) {
            Some(EventArgs::String(string)) => string,
            _ => {
                return Err(Error::Sim(SimError::MissingArgument($key.to_string())));
            }
        }
    }};
}

#[macro_export]
macro_rules! get_node_type_arg {
    ($args:expr, $key:expr) => {{
        match $args.get($key) {
            Some(EventArgs::NodeType(node_type)) => node_type,
            _ => {
                return Err(Error::Sim(SimError::MissingArgument($key.to_string())));
            }
        }
    }};
}

#[macro_export]
macro_rules! get_link_arg {
    ($args:expr, $key:expr) => {{
        match $args.get($key) {
            Some(EventArgs::Link(link)) => link,
            _ => {
                return Err(Error::Sim(SimError::MissingArgument($key.to_string())));
            }
        }
    }};
}

#[macro_export]
macro_rules! get_number_arg {
    ($args:expr, $key:expr) => {{
        match $args.get($key) {
            Some(EventArgs::Number(num)) => num,
            _ => {
                return Err(Error::Sim(SimError::MissingArgument($key.to_string())));
            }
        }
    }};
}

#[macro_export]
macro_rules! get_big_number_arg {
    ($args:expr, $key:expr) => {{
        match $args.get($key) {
            Some(EventArgs::BigNumber(num)) => num,
            _ => {
                return Err(Error::Sim(SimError::MissingArgument($key.to_string())));
            }
        }
    }};
}

#[macro_export]
macro_rules! get_side_arg {
    ($args:expr, $key:expr) => {{
        match $args.get($key) {
            Some(EventArgs::Side(side)) => side,
            _ => {
                return Err(Error::Sim(SimError::MissingArgument($key.to_string())));
            }
        }
    }};
}

// Keep macros to get instances
#[macro_export]
macro_rules! get_qubit_ref_arg {
    ($args:expr, $key:expr) => {{
        match $args.get($key) {
            Some(EventArgs::QubitRef(qubit_ref)) => qubit_ref,
            _ => {
                return Err(Error::Sim(SimError::MissingArgument($key.to_string())));
            }
        }
    }};
}

// Keep macros to get instances
#[macro_export]
macro_rules! get_pairs_arg {
    ($args:expr, $key:expr) => {{
        match $args.get($key) {
            Some(EventArgs::Pairs(pairs)) => pairs,
            _ => {
                return Err(Error::Sim(SimError::MissingArgument($key.to_string())));
            }
        }
    }};
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

pub fn form_word(measurements: Vec<Measurement>) {
    for measurement in measurements {
        print!("{}", measurement.value);
    }
}
