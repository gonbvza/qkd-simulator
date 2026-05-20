use std::collections::HashMap;

use crate::error::SimError;
use crate::{
    core::state::SimulationState,
    error::Error,
    establish_connection, get_node_type_arg, get_string_arg,
    models::{args::EventArgs, detector::Detector},
    nodes::node::Node,
};

pub fn create_node(
    args: &HashMap<String, EventArgs>,
    current_time: i64,
    state: &mut SimulationState,
) -> Result<(), Error> {
    let mut conn = establish_connection();

    let name = get_string_arg!(args, "name");
    let node_type = get_node_type_arg!(args, "node_type");

    let detector = Detector::new()?;
    let node: Node = Node::new(
        &mut conn,
        name.to_owned(),
        node_type.to_string(),
        detector.id,
    )?;

    // Create in local state
    state.upsert_node(node);
    state.upsert_detector(detector);
    Ok(())
}
