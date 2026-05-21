use std::collections::HashMap;

use crate::core::event_loop::EventLoopHandler;
use crate::core::graph::Graph;
use crate::database::link::{get_all_links, get_link};
use crate::database::nodes::{get_all_nodes, get_node_by_id};
use crate::error::Error;
use crate::establish_connection;
use crate::models::args::EventArgs;
use crate::models::event::Event;
use crate::models::links::Link;
use crate::nodes::node::{Node, NodeKind};

pub type Result<T> = std::result::Result<T, Error>;

pub async fn create_node_api(
    name: String,
    node_type: NodeKind,
    handle: &EventLoopHandler,
) -> Result<()> {
    let args: HashMap<String, EventArgs> = HashMap::from([
        (String::from("name"), EventArgs::String(name)),
        (String::from("node_type"), EventArgs::NodeType(node_type)),
    ]);
    let event = Event::new_now("create_node".to_string(), "create_node".to_string(), args);
    handle.push_event(event);
    Ok(())
}

pub async fn create_link_api(
    src_id: i32,
    dst_id: i32,
    distance: i64,
    handle: &EventLoopHandler,
) -> Result<()> {
    let args: HashMap<String, EventArgs> = HashMap::from([
        (String::from("distance"), EventArgs::BigNumber(distance)),
        (String::from("src_id"), EventArgs::Number(src_id)),
        (String::from("dst_id"), EventArgs::Number(dst_id)),
    ]);
    let event = Event::new_now("create_link".to_string(), "create_link".to_string(), args);
    handle.push_event(event);
    Ok(())
}

pub async fn get_nodes_api() -> Result<Vec<Node>> {
    let mut conn = establish_connection();
    let nodes = get_all_nodes(&mut conn)?;
    Ok(nodes)
}

pub async fn get_links_api() -> Result<Vec<Link>> {
    let mut conn = establish_connection();
    let links = get_all_links(&mut conn)?;
    Ok(links)
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
pub async fn start_qkd(
    src_node_id: i32,
    dst_node_id: i32,
    handle: &EventLoopHandler,
) -> Result<()> {
    // Get the nodes
    let mut conn = establish_connection();
    let src_node: Node = get_node_by_id(&mut conn, src_node_id)?;
    let dst_node: Node = get_node_by_id(&mut conn, dst_node_id)?;

    let graph: Graph = Graph::new()?;
    let epr_node: Node = get_node_by_id(&mut conn, graph.get_node_epr(src_node.id, dst_node.id)?)?;

    let src_epr_link: Link = get_link(&mut conn, src_node.id, epr_node.id)?;
    let dst_epr_link: Link = get_link(&mut conn, dst_node.id, epr_node.id)?;

    let args: HashMap<String, EventArgs> = HashMap::from([
        (String::from("src_node_id"), EventArgs::Number(src_node.id)),
        (String::from("dst_node_id"), EventArgs::Number(dst_node.id)),
        (String::from("epr_node_id"), EventArgs::Number(epr_node.id)),
        (String::from("src_epr_link"), EventArgs::Link(src_epr_link)),
        (String::from("dst_epr_link"), EventArgs::Link(dst_epr_link)),
    ]);

    let event = Event::new_now(
        String::from("handle_qkd_event"),
        String::from("handle_qkd_init"),
        args,
    );

    handle.push_event(event);

    Ok(())
}
