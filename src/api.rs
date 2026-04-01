use crate::error::Error;
use crate::establish_connection;
use crate::events::qkd::handle_qkd_init;
use crate::links::Link;
use crate::nodes::common::{client_get_by_id, epr_get_by_id};
use crate::nodes::nodes::{ClientNode, EprNode, NodeKind, NodeType};

pub type Result<T> = std::result::Result<T, Error>;

pub async fn create_node_cli(name: String, node_type: NodeKind) -> Result<NodeType> {
    let mut conn = establish_connection();
    let node: NodeType = match node_type {
        NodeKind::ClientNode => NodeType::ClientNode(ClientNode::new(&mut conn, name)?),
        NodeKind::EprNode => NodeType::EprNode(EprNode::new(&mut conn, name)?),
    };
    Ok(node)
}

pub async fn create_link_cli(node_a: i32, node_b: i32) -> Result<Link> {
    let link = Link::new(100, 0.4, 0.1, node_a, node_b)?;
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
pub async fn start_qkd(sender_id: i32, receiver_id: i32) -> Result<()> {
    // Get the nodes
    let mut conn = establish_connection();
    let mut sender_node: ClientNode = client_get_by_id(&mut conn, sender_id)?;
    let mut receiver_node: ClientNode = client_get_by_id(&mut conn, receiver_id)?;

    // TODO: Get epr node that connects them

    // Get links
    let link_a: Link = epr_node.get_link(&sender_node)?;
    let link_b: Link = epr_node.get_link(&receiver_node)?;
    handle_qkd_init(
        &mut sender_node,
        &mut receiver_node,
        &mut epr_node,
        link_a,
        link_b,
    );

    Ok(())
}
