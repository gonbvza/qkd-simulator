//! Graph representation of current nodes and links.
//!
//! This module builds a graph representation of nodes
//! and uses it to find commom epr nodes.

use std::collections::{HashMap, HashSet};

use crate::{
    database::{link::get_all_links, nodes::get_all_nodes},
    error::{GraphError, LinkError, NodeError},
    establish_connection,
    models::links::Link,
    models::node::{Node, NodeKind},
};

/// Local abstraction of a node used to build the graph
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GraphNode {
    ClientNode(i32),
    EprNode(i32),
}

impl GraphNode {
    /// Create graph node based on the type of node
    fn new(id: i32, node_type: NodeKind) -> GraphNode {
        match node_type {
            NodeKind::ClientNode => GraphNode::ClientNode(id),
            NodeKind::EprNode => GraphNode::EprNode(id),
        }
    }

    /// Unwraps id from the graph node
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

    /// Builds the graph from a list of nodes and links.
    ///
    /// Inserts all nodes and bidirectional connections into the graph.
    /// Returns a [`GraphError`] if a link references a node that doesn't exist
    /// or a node has an unrecognized kind.
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

    /// Returns the ID of the shared EPR node between `src_id` and `dst_id`.
    ///
    /// If multiple EPR nodes are shared, the one with the lowest ID is chosen.
    /// Returns [`GraphError::NoCommonEpr`] if no shared EPR node exists.
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
