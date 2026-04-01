use crate::{
    error::GraphError,
    establish_connection,
    nodes::nodes::{Node, NodeKind},
};

// Instance of an existing node in the graph blueprint
#[derive(Debug)]
pub struct _Node {
    pub id: i32,
    pub node_type: NodeKind,
}

#[derive(Debug)]
pub struct _Link {
    pub id: i32,
    pub node_a: _Node,
    pub node_b: _Node,
}

// Graph blueprint easily navigate through the
// connection of the nodes
#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<_Node>,
    pub connections: Vec<_Link>,
}

impl Graph {
    pub fn new() -> Result<Graph, GraphError> {
        // let mut conn = establish_connection();
        // let curr_nodes: Vec<Node>;
        todo!()
    }
}
