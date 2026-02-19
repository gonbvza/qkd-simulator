use crate::{links::Link, nodes::client::ClientNode};

pub async fn create_node_cli(name: String) {
    let node = ClientNode::new(name);
    println!("Created node with id: {}", node.id);
}

pub async fn create_link_cli(nodea: i32, nodeb: i32) {
    let link = Link::new(100, 0.4, 0.1, nodea, nodeb);
    match link {
        Some(link) => println!("Created node with id: {}", link.id),
        None => (),
    }
}
