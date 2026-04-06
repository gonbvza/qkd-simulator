use std::collections::HashMap;

use crate::database::link::get_link;
use crate::database::nodes::get_node_by_id;
use crate::error::Error;
use crate::establish_connection;
use crate::models::args::EventArgs;
use crate::models::event::Event;
use crate::models::graph::Graph;
use crate::models::links::Link;
use crate::nodes::nodes::{Node, NodeKind};

pub type Result<T> = std::result::Result<T, Error>;

pub async fn create_node_api(name: String, node_type: NodeKind) -> Result<Node> {
    let mut conn = establish_connection();
    let node: Node = Node::new(&mut conn, name, node_type.to_string())?;
    Ok(node)
}

pub async fn create_link_api(src_id: i32, dst_id: i32) -> Result<Link> {
    let link = Link::new(100, 0.4, 0.1, src_id, dst_id)?;
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

    let args: HashMap<String, EventArgs> = HashMap::from([
        (String::from("src_node"), EventArgs::Node(src_node)),
        (String::from("dst_node"), EventArgs::Node(dst_node)),
        (String::from("epr_node"), EventArgs::Node(epr_node)),
        (String::from("src_epr_link"), EventArgs::Link(src_epr_link)),
        (String::from("dst_epr_link"), EventArgs::Link(dst_epr_link)),
    ]);

    // TODO: Calculate timestamp
    Event::new_and_push(
        String::from("handle_qkd_event"),
        String::from("handle_qkd_init"),
        args,
        12,
    );

    Ok(())
}
