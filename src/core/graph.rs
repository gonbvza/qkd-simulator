use std::collections::{HashMap, HashSet};

use crate::{
    database::{link::get_all_links, nodes::get_all_nodes},
    error::{GraphError, LinkError, NodeError},
    establish_connection,
    models::links::Link,
    models::node::{Node, NodeKind},
};

// Local NodeKind enum that servers as wrapper for nodes ids
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GraphNode {
    ClientNode(i32),
    EprNode(i32),
}

impl GraphNode {
    fn new(id: i32, node_type: NodeKind) -> GraphNode {
        match node_type {
            NodeKind::ClientNode => GraphNode::ClientNode(id),
            NodeKind::EprNode => GraphNode::EprNode(id),
        }
    }

    fn get_id(&self) -> i32 {
        match self {
            GraphNode::ClientNode(id) => *id,
            GraphNode::EprNode(id) => *id,
        }
    }
}

// Graph blueprint easily navigate through the
// connection of the nodes
#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, GraphNode>,
    pub connections: HashMap<i32, HashSet<GraphNode>>,
}

impl Graph {
    pub fn new() -> Result<Graph, GraphError> {
        let mut conn = establish_connection();
        Self::from_data(get_all_nodes(&mut conn)?, get_all_links(&mut conn)?)
    }

    pub fn from_data(curr_nodes: Vec<Node>, curr_links: Vec<Link>) -> Result<Graph, GraphError> {
        let mut graph: Graph = Graph {
            nodes: HashMap::new(),
            connections: HashMap::new(),
        };
        // Instantiate nodes
        for node in curr_nodes.iter() {
            graph.nodes.insert(
                node.id,
                GraphNode::new(
                    node.id,
                    node.node_type
                        .parse::<NodeKind>()
                        .map_err(|_| NodeError::NotValidKind(node.node_type.clone()))?,
                ),
            );
            graph.connections.insert(node.id, HashSet::new());
        }
        //Instantiate connections
        for link in curr_links.iter() {
            let src_id: &GraphNode = graph
                .nodes
                .get(&link.src_id)
                .ok_or(LinkError::MissingNode(link.src_id))?;
            let dst_id: &GraphNode = graph
                .nodes
                .get(&link.dst_id)
                .ok_or(LinkError::MissingNode(link.dst_id))?;
            graph
                .connections
                .get_mut(&link.src_id)
                .ok_or(LinkError::MissingNode(link.src_id))?
                .insert(dst_id.clone());
            graph
                .connections
                .get_mut(&link.dst_id)
                .ok_or(LinkError::MissingNode(link.dst_id))?
                .insert(src_id.clone());
        }
        Ok(graph)
    }

    /// Function to find related epr node between two nodes.
    ///
    /// The epr node with the lowest id is chosen if they share any
    ///
    /// Arguments
    /// * src_id - The node id of the first node
    /// * dst_id - The node id of the second node
    ///
    /// Returns
    /// * Node   - Instance of the epr node,
    pub fn get_node_epr(&self, src_id: i32, dst_id: i32) -> Result<i32, GraphError> {
        let a_neighbors: &HashSet<GraphNode> = self
            .connections
            .get(&src_id)
            .ok_or(LinkError::MissingNode(src_id))?;
        let b_neighbors: &HashSet<GraphNode> = self
            .connections
            .get(&dst_id)
            .ok_or(LinkError::MissingNode(dst_id))?;

        let common_epr = b_neighbors
            .iter()
            .filter(|node| a_neighbors.contains(node))
            .filter(|node| matches!(node, GraphNode::EprNode(_)))
            .min();

        match common_epr {
            Some(epr) => Ok(epr.get_id()),
            None => Err(GraphError::NoCommonEpr(src_id, dst_id)),
        }
    }
}
