use crate::error::Error;
use crate::nodes::epr::EprNode;
use crate::nodes::{NodeKind, NodeType};
use crate::{links::Link, nodes::client::ClientNode};

pub type Result<T> = std::result::Result<T, Error>;

pub async fn create_node_cli(name: String, node_type: NodeKind) -> Result<NodeType> {
    let node: NodeType = match node_type {
        NodeKind::ClientNode => NodeType::ClientNode(ClientNode::new(name)?),
        NodeKind::EprNode => NodeType::EprNode(EprNode::new(name)?),
    };
    Ok(node)
}

pub async fn create_link_cli(nodea: i32, nodeb: i32) -> Result<Link> {
    let link = Link::new(100, 0.4, 0.1, nodea, nodeb)?;
    Ok(link)
}
