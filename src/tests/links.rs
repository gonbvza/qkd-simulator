use crate::{establish_connection, models::links::Link, schema};
use diesel::prelude::*;

fn clean_tables(conn: &mut PgConnection) {
    diesel::delete(schema::entangled_pair::table)
        .execute(conn)
        .unwrap();
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
fn link_creation_fails_if_node_missing() {
    let mut conn = establish_connection();
    clean_tables(&mut conn);

    let detector_id = insert_detector(&mut conn);
    let node_id: i32 = diesel::insert_into(schema::nodes::table)
        .values((
            schema::nodes::name.eq("test-node"),
            schema::nodes::detector_id.eq(detector_id),
        ))
        .returning(schema::nodes::id)
        .get_result(&mut conn)
        .unwrap();

    let missing_node = node_id + 999;
    let link = Link::new(100, 0.1, 0.01, node_id, missing_node);
    assert!(!link.is_ok());
}

#[test]
fn link_creation_succeeds_with_valid_nodes() {
    let mut conn = establish_connection();
    clean_tables(&mut conn);

    let detector_id = insert_detector(&mut conn);
    let src_id: i32 = diesel::insert_into(schema::nodes::table)
        .values((
            schema::nodes::name.eq("node-a"),
            schema::nodes::detector_id.eq(detector_id),
        ))
        .returning(schema::nodes::id)
        .get_result(&mut conn)
        .unwrap();
    let dst_id: i32 = diesel::insert_into(schema::nodes::table)
        .values((
            schema::nodes::name.eq("node-b"),
            schema::nodes::detector_id.eq(detector_id),
        ))
        .returning(schema::nodes::id)
        .get_result(&mut conn)
        .unwrap();

    let link = Link::new(42, 0.25, 0.001, src_id, dst_id).expect("Link should be created");

    assert_eq!(link.length, 42);
    assert_eq!(link.attenuation, 0.25);
    assert_eq!(link.error_rate, 0.001);
    assert_eq!(link.src_id, src_id);
    assert_eq!(link.dst_id, dst_id);
    assert_eq!(link.next_available_time, 0);
}
