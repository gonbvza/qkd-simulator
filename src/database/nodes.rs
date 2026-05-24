use diesel::prelude::*;

use crate::{
    error::{map_db_error, NodeError},
    models::node::Node,
    schema,
};

pub fn create_node(
    conn: &mut PgConnection,
    name: &str,
    node_type: &str,
    detector_id: i32,
) -> Result<Node, NodeError> {
    diesel::insert_into(schema::nodes::table)
        .values((
            schema::nodes::name.eq(name),
            schema::nodes::node_type.eq(node_type),
            schema::nodes::detector_id.eq(detector_id),
        ))
        .get_result(conn)
        .map_err(|e| map_db_error(name.to_string(), e))
}

pub fn get_node_by_id(conn: &mut PgConnection, node_id: i32) -> Result<Node, NodeError> {
    schema::nodes::table
        .filter(schema::nodes::id.eq(node_id))
        .first(conn)
        .map_err(|e| map_db_error(node_id.to_string(), e))
}

pub fn get_all_nodes(conn: &mut PgConnection) -> Result<Vec<Node>, NodeError> {
    let nodes: Vec<Node> = schema::nodes::table.load(conn)?;
    return Ok(nodes);
}
