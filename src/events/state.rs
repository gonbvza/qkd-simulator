use std::collections::HashMap;

use crate::core::event_loop::EventLoopHandler;
use crate::error::SimError;
use crate::models::links::Link;
use crate::{
    core::state::SimulationState,
    error::Error,
    establish_connection, get_node_type_arg, get_string_arg,
    models::{args::EventArgs, detector::Detector},
    nodes::node::Node,
};
use crate::{get_big_number_arg, get_number_arg};

pub fn create_node(
    args: &HashMap<String, EventArgs>,
    _current_time: i64,
    state: &mut SimulationState,
    _handle: &EventLoopHandler,
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

pub fn create_link(
    args: &HashMap<String, EventArgs>,
    _current_time: i64,
    state: &mut SimulationState,
    _handle: &EventLoopHandler,
) -> Result<(), Error> {
    let mut conn = establish_connection();

    let distance = get_big_number_arg!(args, "distance").to_owned();
    let src_id = get_number_arg!(args, "src_id").to_owned();
    let dst_id = get_number_arg!(args, "dst_id").to_owned();

    let link = Link::new(&mut conn, distance, 0.4, 0.1, src_id, dst_id)?;

    // Create in local state
    state.upsert_link(link);
    Ok(())
}
