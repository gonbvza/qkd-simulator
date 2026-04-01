use std::fmt;

use crate::error::{CliError, NodeError};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Node {
    pub id: i32,
    pub name: String,
    pub in_use: bool,
    pub measurements: i64,
    pub node_type: String,
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
            "1" => Ok(NodeKind::EprNode),
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
