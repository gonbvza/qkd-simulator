use std::collections::HashMap;

use crate::models::{args::EventArgs, event::EventLoop};

mod models;
mod registry;
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

fn main() {
    // 1. Start priority heap event loop and store function instances
    let mut event_loop = EventLoop::new();
    event_loop.instantiate_functions();

    // 2.  Start event loop
    event_loop.run_loop();
}
