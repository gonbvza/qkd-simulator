use crate::nodes::client::ClientNode;

pub async fn create_node_cli(name: String) {
    let node = ClientNode::new(name);
    println!("Created node with id: {}", node.id);
}
