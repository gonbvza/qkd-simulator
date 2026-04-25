use std::fmt;

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

impl TryFrom<&String> for NodeKind {
    type Error = NodeError;
    fn try_from(s: &String) -> Result<Self, Self::Error> {
        match s.trim() {
            "0" => Ok(NodeKind::ClientNode),
            "client" => Ok(NodeKind::ClientNode),
            "1" => Ok(NodeKind::EprNode),
            "epr" => Ok(NodeKind::ClientNode),
            _ => Err(NodeError::NotValidKind(s.to_string())),
        }
    }
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeKind::ClientNode => write!(f, "0"),
            NodeKind::EprNode => write!(f, "1"),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = NodeKind::try_from(&self.node_type)
            .map(|k| k.to_string())
            .unwrap_or_else(|_| format!("Unknown ({})", self.node_type));
        let owner = if let Some(owner) = self.locked_by {
            {
                format!("{}", owner)
            }
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
