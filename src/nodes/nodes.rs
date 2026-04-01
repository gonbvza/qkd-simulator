use crate::error::CliError;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EprNode {
    pub id: i32,
    pub name: String,
    pub in_use: bool,
}

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClientNode {
    pub id: i32,
    pub name: String,
    pub in_use: bool,
    pub measurements: i64,
    pub node_type: String,
}

pub enum NodeKind {
    ClientNode = 0,
    EprNode = 1,
}

#[derive(Clone, Debug)]
pub enum NodeType {
    ClientNode(ClientNode),
    EprNode(EprNode),
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

pub trait NodeUsage {
    fn get_id(&self) -> i32;
    fn set_in_use(&mut self, value: bool);
}
