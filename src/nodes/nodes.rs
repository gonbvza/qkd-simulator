use crate::{
    error::{CliError, NodeError},
    establish_connection,
    models::measurement::Measurement,
    nodes::repository::{create_client_node, create_epr_node},
    schema::{self},
};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EprNode {
    pub id: i32,
    pub name: String,
    pub in_use: bool,
}

#[derive(Queryable, Selectable)]
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

impl ClientNode {
    pub fn new(conn: &mut PgConnection, name: String) -> Result<ClientNode, NodeError> {
        create_client_node(conn, &name)
    }

    pub fn get_measurements(&self) -> Result<Vec<Measurement>, NodeError> {
        let mut conn = establish_connection();

        let measurements: Vec<Measurement> = schema::measurements::table
            .select(Measurement::as_select())
            .filter(schema::measurements::node_id.eq(self.id))
            .load(&mut conn)?;

        Ok(measurements)
    }
}

impl EprNode {
    pub fn new(conn: &mut PgConnection, name: String) -> Result<EprNode, NodeError> {
        create_epr_node(conn, &name)
    }
}
