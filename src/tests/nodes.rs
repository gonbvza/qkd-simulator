use diesel::prelude::*;

use crate::{
    database::nodes::{get_node_by_id, get_node_by_name},
    establish_connection,
    nodes::node::{Node, NodeKind},
    schema,
};

#[test]
fn test_create_client_node() {
    let mut conn = establish_connection();
    // Clean tables for deterministic test
    diesel::delete(schema::links::table)
        .execute(&mut conn)
        .unwrap();
    diesel::delete(schema::nodes::table)
        .execute(&mut conn)
        .unwrap();

    let node = Node::new(
        &mut conn,
        "test_client".to_string(),
        NodeKind::ClientNode.to_string(),
    );
    assert!(node.is_ok());

    let node_by_name = get_node_by_name(&mut conn, "test_client");
    assert!(node_by_name.is_ok());
}
