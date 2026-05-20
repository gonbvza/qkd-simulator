use std::collections::HashMap;
use std::sync::Arc;

use crate::core::event_loop::EventLoop;
use crate::core::graph::Graph;
use crate::database::link::get_link;
use crate::database::nodes::get_node_by_id;
use crate::error::Error;
use crate::establish_connection;
use crate::models::args::EventArgs;
use crate::models::detector::Detector;
use crate::models::links::Link;
use crate::nodes::node::{Node, NodeKind};

pub type Result<T> = std::result::Result<T, Error>;

pub async fn create_node_api(name: String, node_type: NodeKind) -> Result<()> {
    let args: HashMap<String, EventArgs> = HashMap::from([
        (String::from("name"), EventArgs::String(name)),
        (String::from("node_type"), EventArgs::NodeType(node_type)),
    ]);
    let current_time = {
        let loop_pair = Arc::clone(&*EventLoop::instance());
        let (event_loop, _) = &*loop_pair;

        let mut guard = event_loop.lock().unwrap();
        guard.get_current_time()
    };
    EventLoop::new_and_push(
        "create_node".to_string(),
        "create_node".to_string(),
        args,
        // Give priority to this type of events
        current_time + 1,
    );
    Ok(())
}

pub async fn create_link_api(src_id: i32, dst_id: i32, distance: i64) -> Result<Link> {
    let link = Link::new(distance, 0.4, 0.1, src_id, dst_id)?;
    Ok(link)
}

/// Initiates a QKD session between two client nodes via an EPR source.
///
/// This is the top-level entry point for starting a key distribution session.
/// It validates that the three nodes exist in the repository, retrieves the
/// sender node, and delegates to [`Client::start_qkd`].
///
/// # Arguments
/// * `sender_id`   - The [`NodeId`] of the client node initiating the session
/// * `receiver_id` - The [`NodeId`] of the destination client node
/// * `epr_id`      - The [`NodeId`] of the EPR node that will emit entangled pairs
///
/// # Errors
/// Returns an error if any of the three node IDs are not found in the repository
pub async fn start_qkd(src_node: Node, dst_node: Node) -> Result<()> {
    // Get the nodes
    let mut conn = establish_connection();

    let graph: Graph = Graph::new()?;
    let epr_node: Node = get_node_by_id(&mut conn, graph.get_node_epr(src_node.id, dst_node.id)?)?;

    let src_epr_link: Link = get_link(&mut conn, src_node.id, epr_node.id)?;
    let dst_epr_link: Link = get_link(&mut conn, dst_node.id, epr_node.id)?;
    let current_time = {
        let loop_pair = Arc::clone(&*EventLoop::instance());
        let (event_loop, _) = &*loop_pair;

        let mut guard = event_loop.lock().unwrap();
        guard.get_current_time()
    };
    let delay: i64 = src_epr_link.propagation_delay_us();

    let args: HashMap<String, EventArgs> = HashMap::from([
        (String::from("src_node"), EventArgs::Node(src_node)),
        (String::from("dst_node"), EventArgs::Node(dst_node)),
        (String::from("epr_node"), EventArgs::Node(epr_node)),
        (String::from("src_epr_link"), EventArgs::Link(src_epr_link)),
        (String::from("dst_epr_link"), EventArgs::Link(dst_epr_link)),
    ]);

    // TODO: Calculate timestamp
    EventLoop::new_and_push(
        String::from("handle_qkd_event"),
        String::from("handle_qkd_init"),
        args,
        current_time + delay,
    );

    Ok(())
}
