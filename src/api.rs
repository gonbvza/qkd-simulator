use crate::database::nodes::get_node_by_id;
use crate::error::Error;
use crate::establish_connection;
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
pub async fn start_qkd_api(sender_id: i32, receiver_id: i32) -> Result<()> {
    // Get the nodes
    let mut conn = establish_connection();
    let mut sender_node: Node = get_node_by_id(&mut conn, sender_id)?;
    let mut receiver_node: Node = get_node_by_id(&mut conn, receiver_id)?;

    // TODO: Get epr node that connects them
    // TODO: GET LINKS
    // TODO: CALL HANDLE QKD INIT in the event loop

    Ok(())
}
