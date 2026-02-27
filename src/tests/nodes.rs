use diesel::prelude::*;

use crate::{
    establish_connection,
    links::Link,
    nodes::{client::ClientNode, epr::EprNode},
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

    let node = ClientNode::new("test_client".to_string());

    assert!(node.is_ok());

    let node_by_name = ClientNode::get_by_name("test_client");

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

    let node = EprNode::new("test_epr".to_string());

    assert!(node.is_ok());

    let node_by_name = EprNode::get_by_name("test_epr");

    assert!(node_by_name.is_ok());
}
