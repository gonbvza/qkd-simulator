use crate::{
    establish_connection,
    links::Link,
    nodes::{
        common::client_get_by_name,
        nodes::{ClientNode, EprNode},
    },
    schema,
};
use diesel::prelude::*;

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

    let node = ClientNode::new(&mut conn, "test_client".to_string());
    assert!(node.is_ok());

    let node_by_name = client_get_by_name(&mut conn, "test_client");
    assert!(node_by_name.is_ok());
}

#[test]
fn test_create_epr_node() {
    let mut conn = establish_connection();
    // Clean tables for deterministic test
    diesel::delete(schema::links::table)
        .execute(&mut conn)
        .unwrap();
    diesel::delete(schema::nodes::table)
        .execute(&mut conn)
        .unwrap();

    let node = EprNode::new(&mut conn, "test_epr".to_string());
    assert!(node.is_ok());
}
