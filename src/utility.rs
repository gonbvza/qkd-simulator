use std::{collections::HashMap, io};

use crate::{
    database::{entangled_pair::get_pair_by_id, nodes::get_node_by_id},
    error::{Error, PairError, SimError},
    establish_connection,
    models::{args::EventArgs, measurement::Measurement, qubit_ref::QubitRefSide},
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

// Keep functions to retrive instances from db
pub fn get_node_arg(args: &HashMap<String, EventArgs>, key: &str) -> Result<Node, Error> {
    let mut conn = establish_connection();
    match args.get(key) {
        Some(EventArgs::Node(node_id)) => {
            let node = get_node_by_id(&mut conn, node_id.to_owned())?;
            Ok(node)
        }
        _ => {
            return Err(SimError::MissingArgument(key.to_string()).into());
        }
    }
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

pub fn is_first(entangled_pair_id: i32, side: QubitRefSide) -> Result<bool, PairError> {
    let mut conn = establish_connection();
    let entangled_pair = get_pair_by_id(&mut conn, entangled_pair_id)?;
    match side {
        QubitRefSide::Source => Ok(entangled_pair.dst_measured.is_none()),
        QubitRefSide::Destination => Ok(entangled_pair.src_measured.is_none()),
    }
}

pub fn form_word(measurements: Vec<Measurement>) {
    for measurement in measurements {
        print!("{}", measurement.value);
    }
}
