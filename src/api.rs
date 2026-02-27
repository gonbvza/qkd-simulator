use crate::error::Error;
use crate::{links::Link, nodes::client::ClientNode};

pub type Result<T> = std::result::Result<T, Error>;

pub async fn create_node_cli(name: String) -> Result<()> {
    let node = ClientNode::new(name);
    println!("Created node with id: {}", node?.id);
    Ok(())
}

pub async fn create_link_cli(nodea: i32, nodeb: i32) -> Result<Link> {
    let link = Link::new(100, 0.4, 0.1, nodea, nodeb)?;
    Ok(link)
}
