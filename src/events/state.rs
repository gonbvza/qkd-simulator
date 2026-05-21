use crate::core::event_loop::EventLoopHandler;
use crate::models::event_types::EventPayload;
use crate::models::links::Link;
use crate::{
    core::state::SimulationState, error::Error, establish_connection, models::detector::Detector,
    nodes::node::Node,
};

pub fn create_node(
    payload: EventPayload,
    _current_time: i64,
    state: &mut SimulationState,
    _handle: &EventLoopHandler,
) -> Result<(), Error> {
    let mut conn = establish_connection();
    let EventPayload::CreateNode(args) = payload else {
        return Err(Error::WrongArgs());
    };

    let detector = Detector::new()?;
    let node: Node = Node::new(
        &mut conn,
        args.name,
        args.node_type.to_string(),
        detector.id,
    )?;

    // Create in local state
    state.upsert_node(node);
    state.upsert_detector(detector);
    Ok(())
}

pub fn create_link(
    payload: EventPayload,
    _current_time: i64,
    state: &mut SimulationState,
    _handle: &EventLoopHandler,
) -> Result<(), Error> {
    let mut conn = establish_connection();
    let EventPayload::CreateLink(args) = payload else {
        return Err(Error::WrongArgs());
    };

    let link = Link::new(&mut conn, args.distance, 0.4, 0.1, args.src_id, args.dst_id)?;

    // Create in local state
    state.upsert_link(link);
    Ok(())
}
