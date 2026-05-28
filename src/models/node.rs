use std::fmt;

use crate::database::nodes::create_node;
use crate::error::{CliError, NodeError};
use diesel::prelude::*;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeKind {
    ClientNode = 0,
    EprNode = 1,
}

impl std::str::FromStr for NodeKind {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "0" => Ok(NodeKind::ClientNode),
            "1" => Ok(NodeKind::EprNode),
            _ => Err(CliError::NotValidInput(s.to_string())),
        }
    }
}

// Prefer parsing via `FromStr`/`parse()`; the older `TryFrom<&String>` was
// redundant and less ergonomic.

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeKind::ClientNode => write!(f, "0"),
            NodeKind::EprNode => write!(f, "1"),
        }
    }
}

impl Node {
    pub fn new(
        conn: &mut PgConnection,
        name: String,
        node_type: String,
        detector_id: i32,
    ) -> Result<Node, NodeError> {
        create_node(conn, &name, &node_type, detector_id)
    }

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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = self
            .node_type
            .parse::<NodeKind>()
            .map(|k| k.to_string())
            .unwrap_or_else(|_| format!("Unknown ({})", self.node_type));
        let owner = if let Some(owner) = self.locked_by {
            format!("{}", owner)
        } else {
            "No".to_string()
        };
        write!(
            f,
            "[{}] {} | Type: {} | In Use: {} | ",
            self.id, self.name, kind, owner
        )
    }
}
