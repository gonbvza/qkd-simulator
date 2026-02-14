use diesel::prelude::*;

use crate::{establish_connection, links::Link, schema};

#[test]
fn link_creation_fails_if_node_missing() {
    let mut conn = establish_connection();

    // Clean tables for deterministic test
    diesel::delete(schema::links::table)
        .execute(&mut conn)
        .unwrap();
    diesel::delete(schema::nodes::table)
        .execute(&mut conn)
        .unwrap();

    // Insert ONLY one node
    let node_id: i32 = diesel::insert_into(schema::nodes::table)
        .values(schema::nodes::name.eq("test-node"))
        .returning(schema::nodes::id)
        .get_result(&mut conn)
        .unwrap();

    // Second node does NOT exist
    let missing_node = node_id + 999;

    let link = Link::new(
        100,  // length
        0.1,  // attenuation
        0.01, // error
        node_id,
        missing_node,
    );

    assert!(link.is_none());
}

#[test]
fn link_creation_succeeds_with_valid_nodes() {
    let mut conn = establish_connection();

    // Clean tables for deterministic test
    diesel::delete(schema::links::table)
        .execute(&mut conn)
        .unwrap();
    diesel::delete(schema::nodes::table)
        .execute(&mut conn)
        .unwrap();

    // Insert two nodes
    let node_a: i32 = diesel::insert_into(schema::nodes::table)
        .values(schema::nodes::name.eq("node-a"))
        .returning(schema::nodes::id)
        .get_result(&mut conn)
        .unwrap();

    let node_b: i32 = diesel::insert_into(schema::nodes::table)
        .values(schema::nodes::name.eq("node-b"))
        .returning(schema::nodes::id)
        .get_result(&mut conn)
        .unwrap();

    let length = 42;
    let attenuation = 0.25;
    let error = 0.001;

    let link =
        Link::new(length, attenuation, error, node_a, node_b).expect("Link should be created");

    // Validate fields
    assert_eq!(link.length, length);
    assert_eq!(link.attenuation, attenuation);
    assert_eq!(link.error, error);
    assert_eq!(link.nodea, node_a);
    assert_eq!(link.nodeb, node_b);
    assert_eq!(link.next_available_time, 0);
}
