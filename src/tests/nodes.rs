use crate::schema;
use crate::{
    error::NodeError,
    establish_connection,
    models::node::{Node, NodeKind},
};
use diesel::prelude::*;
use diesel::PgConnection;

fn clean_db(conn: &mut PgConnection) {
    diesel::delete(schema::links::table).execute(conn).unwrap();
    diesel::delete(schema::nodes::table).execute(conn).unwrap();
}

fn insert_detector(conn: &mut PgConnection) -> i32 {
    diesel::insert_into(schema::detector::table)
        .values((
            schema::detector::resolution_ps.eq(0),
            schema::detector::cooldown_ps.eq(0),
            schema::detector::dark_count_rate.eq(0),
            schema::detector::last_detection_time.eq(0),
        ))
        .returning(schema::detector::id)
        .get_result(conn)
        .unwrap()
}

#[test]
fn test_create_client_node() {
    let mut conn = establish_connection();
    clean_db(&mut conn);
    // Clean tables for deterministic test
    let detector_id = insert_detector(&mut conn);
    let _ = Node::new(
        &mut conn,
        "test_client".to_string(),
        NodeKind::ClientNode.to_string(),
        detector_id,
    )
    .unwrap();
}

#[test]
fn test_acquire_node() {
    let mut conn = establish_connection();
    clean_db(&mut conn);
    // Clean tables for deterministic test
    let detector_id = insert_detector(&mut conn);
    let mut node = Node::new(
        &mut conn,
        "test_client".to_string(),
        NodeKind::ClientNode.to_string(),
        detector_id,
    )
    .expect("Node creation failed");

    // 1. Acquire lock
    assert!(node.try_acquire(1));
    // 2. Test locked by
    assert_eq!(node.locked_by, Some(1));
    // 3. Release lock by wrong procces
    assert_eq!(node.release(2).unwrap_err(), NodeError::NotAuthorized(2));
    // 4. Release lock and check
    assert!(node.release(1).is_ok());
    assert_eq!(node.locked_by, None);
}
