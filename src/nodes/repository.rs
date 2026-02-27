use diesel::prelude::*;

use crate::{
    error::{map_db_error, NodeError},
    nodes::nodes::ClientNode,
    nodes::nodes::EprNode,
    schema, settings,
};

pub fn create_client_node(conn: &mut PgConnection, name: &str) -> Result<ClientNode, NodeError> {
    diesel::insert_into(schema::nodes::table)
        .values(schema::nodes::name.eq(name.clone()))
        .get_result(conn)
        .map_err(|e| map_db_error(name.to_string(), e))
}

pub fn create_epr_node(conn: &mut PgConnection, name: &str) -> Result<EprNode, NodeError> {
    diesel::insert_into(schema::nodes::table)
        .values((
            schema::nodes::name.eq(name),
            schema::nodes::node_type.eq(settings::EPR_NODE.clone()),
        ))
        .returning((
            schema::nodes::id,
            schema::nodes::name,
            schema::nodes::in_use,
        ))
        .get_result(conn)
        .map_err(|e| map_db_error(name.to_string(), e))
}

pub fn client_get_by_id(conn: &mut PgConnection, node_id: i32) -> Result<ClientNode, NodeError> {
    schema::nodes::table
        .filter(schema::nodes::id.eq(node_id))
        .select(ClientNode::as_select())
        .first(conn)
        .map_err(|e| map_db_error(node_id.to_string(), e))
}

pub fn client_get_by_name(
    conn: &mut PgConnection,
    node_name: &str,
) -> Result<ClientNode, NodeError> {
    schema::nodes::table
        .filter(schema::nodes::name.eq(node_name))
        .select(ClientNode::as_select())
        .first(conn)
        .map_err(|e| map_db_error(node_name.to_string(), e))
}

pub fn client_get_in_use(conn: &mut PgConnection) -> Result<Vec<ClientNode>, NodeError> {
    schema::nodes::table
        .filter(schema::nodes::in_use.eq(true))
        .select(ClientNode::as_select())
        .load(conn)
        .map_err(|e| map_db_error("get_in_use".to_string(), e))
}
