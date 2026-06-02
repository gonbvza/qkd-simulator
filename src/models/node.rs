use std::fmt;

use diesel::prelude::*;

use crate::{
    database::nodes::create_node,
    error::{CliError, NodeError},
};

/// A network node, either a client or an EPR source.
///
/// Tracks which process currently holds the node via `locked_by`,
/// enforcing that only one QKD session uses it at a time.
#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Node {
    pub id: i32,
    pub name: String,
    pub locked_by: Option<i32>,
    pub node_type: String,
    pub detector_id: i32,
}

/// Differs between client nodes and EPR source nodes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeKind {
    ClientNode = 0,
    EprNode = 1,
    Mallory = 2,
}

impl Node {
    /// Creates a new node in the database.
    pub fn new(
        conn: &mut PgConnection,
        name: String,
        node_type: String,
        detector_id: i32,
    ) -> Result<Node, NodeError> {
        create_node(conn, &name, &node_type, detector_id)
    }

    /// Attempts to lock the node for the given process.
    ///
    /// Returns `true` if the node was free or already owned by the same process,
    /// `false` if it is held by a different process.
    pub fn try_acquire(&mut self, process_id: i32) -> bool {
        match self.locked_by {
            None => {
                self.locked_by = Some(process_id);
                true
            }
            Some(owner) if owner == process_id => true,
            _ => false,
        }
    }

    /// Releases the node lock held by the given process.
    ///
    /// Returns [`NodeError::NotAuthorized`] if the node is locked by a different process.
    pub fn release(&mut self, process_id: i32) -> Result<(), NodeError> {
        if self.locked_by == None {
            return Ok(());
        }
        if self.locked_by != Some(process_id) {
            return Err(NodeError::NotAuthorized(process_id));
        }
        self.locked_by = None;
        Ok(())
    }
}

impl std::str::FromStr for NodeKind {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "0" => Ok(NodeKind::ClientNode),
            "1" => Ok(NodeKind::EprNode),
            "2" => Ok(NodeKind::Mallory),
            _ => Err(CliError::NotValidInput(s.to_string())),
        }
    }
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeKind::ClientNode => write!(f, "0"),
            NodeKind::EprNode => write!(f, "1"),
            NodeKind::Mallory => write!(f, "2"),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = self
            .node_type
            .parse::<NodeKind>()
            .map(|k| k.to_string())
            .unwrap_or_else(|_| format!("Unknown ({})", self.node_type));
        let owner = match self.locked_by {
            Some(owner) => format!("{}", owner),
            None => "No".to_string(),
        };
        write!(
            f,
            "[{}] {} | Type: {} | In Use: {} | ",
            self.id, self.name, kind, owner
        )
    }
}
