use std::sync::mpsc::{channel, Receiver};

use diesel::prelude::*;

use crate::{
    api::{create_link_api, create_node_api, start_qkd},
    core::event_loop::EventLoopHandler,
    establish_connection,
    models::{
        event::{Event, EventTime},
        event_types::{EventName, EventPayload},
        links::Link,
        node::{Node, NodeKind},
    },
    schema,
};
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

fn insert_node(conn: &mut PgConnection, name: &str, kind: NodeKind) -> Node {
    let detector_id = insert_detector(conn);
    Node::new(conn, name.to_string(), kind.to_string(), detector_id).unwrap()
}

fn insert_link(conn: &mut PgConnection, src_id: i32, dst_id: i32) -> Link {
    Link::new(conn, 100, 0.4, 0.1, src_id, dst_id, true).unwrap()
}

fn recv_event(rx: &Receiver<Event>) -> Event {
    rx.recv().expect("expected event on channel")
}

fn assert_now_timestamp(event: &Event) {
    assert!(matches!(event.timestamp, EventTime::Now));
}

fn assert_create_node_event(event: Event, expected_name: &str, expected_kind: NodeKind) {
    assert_eq!(event.name, EventName::CreateNode);
    assert_now_timestamp(&event);

    match event.payload {
        EventPayload::CreateNode(payload) => {
            assert_eq!(payload.name, expected_name);
            assert_eq!(payload.node_type, expected_kind);
        }
        other => panic!("unexpected event payload: {other:?}"),
    }
}

fn assert_create_link_event(event: Event, src_id: i32, dst_id: i32, distance: i64) {
    assert_eq!(event.name, EventName::CreateLink);
    assert_now_timestamp(&event);

    match event.payload {
        EventPayload::CreateLink(payload) => {
            assert_eq!(payload.src_id, src_id);
            assert_eq!(payload.dst_id, dst_id);
            assert_eq!(payload.distance, distance);
        }
        other => panic!("unexpected event payload: {other:?}"),
    }
}

fn assert_handle_qkd_init_event(
    event: Event,
    src_node_id: i32,
    dst_node_id: i32,
    epr_node_id: i32,
    src_epr_link_id: i32,
    dst_epr_link_id: i32,
) {
    assert_eq!(event.name, EventName::HandleQkdInit);
    assert_now_timestamp(&event);

    match event.payload {
        EventPayload::HandleQkdInit(payload) => {
            assert_eq!(payload.src_node_id, src_node_id);
            assert_eq!(payload.dst_node_id, dst_node_id);
            assert_eq!(payload.epr_node_id, epr_node_id);
            assert_eq!(payload.src_epr_link_id, src_epr_link_id);
            assert_eq!(payload.dst_epr_link_id, dst_epr_link_id);
        }
        other => panic!("unexpected event payload: {other:?}"),
    }
}

#[tokio::test]
async fn test_create_node_api_enqueues_client_event() {
    let (tx, rx) = channel();
    let handler = EventLoopHandler::new(tx);

    create_node_api("alice".to_string(), NodeKind::ClientNode, &handler)
        .await
        .unwrap();

    let event = recv_event(&rx);
    assert_create_node_event(event, "alice", NodeKind::ClientNode);
}

#[tokio::test]
async fn test_create_node_api_enqueues_epr_event() {
    let (tx, rx) = channel();
    let handler = EventLoopHandler::new(tx);

    create_node_api("epr-source".to_string(), NodeKind::EprNode, &handler)
        .await
        .unwrap();

    let event = recv_event(&rx);
    assert_create_node_event(event, "epr-source", NodeKind::EprNode);
}

#[tokio::test]
async fn test_create_link_api_enqueues_link_event() {
    let (tx, rx) = channel();
    let handler = EventLoopHandler::new(tx);

    create_link_api(12, 34, 100, true, &handler).await.unwrap();

    let event = recv_event(&rx);
    assert_create_link_event(event, 12, 34, 100);
}

#[tokio::test]
async fn test_start_qkd_enqueues_init_event_with_valid_topology() {
    let mut conn = establish_connection();
    clean_db(&mut conn);

    let src = insert_node(&mut conn, "qkd-src", NodeKind::ClientNode);
    let dst = insert_node(&mut conn, "qkd-dst", NodeKind::ClientNode);
    let epr = insert_node(&mut conn, "qkd-epr", NodeKind::EprNode);
    let src_epr_link = insert_link(&mut conn, src.id, epr.id);
    let dst_epr_link = insert_link(&mut conn, dst.id, epr.id);

    let (tx, rx) = channel();
    let handler = EventLoopHandler::new(tx);

    start_qkd(src.id, dst.id, &handler).await.unwrap();

    let event = recv_event(&rx);
    assert_handle_qkd_init_event(
        event,
        src.id,
        dst.id,
        epr.id,
        src_epr_link.id,
        dst_epr_link.id,
    );
}

#[tokio::test]
async fn test_start_qkd_does_not_enqueue_when_epr_link_is_missing() {
    let mut conn = establish_connection();
    clean_db(&mut conn);

    let src = insert_node(&mut conn, "no-link-src", NodeKind::ClientNode);
    let dst = insert_node(&mut conn, "no-link-dst", NodeKind::ClientNode);

    let (tx, rx) = channel();
    let handler = EventLoopHandler::new(tx);

    let result = start_qkd(src.id, dst.id, &handler).await;

    assert!(result.is_err());
    assert!(rx.try_recv().is_err());
}

#[tokio::test]
async fn test_start_qkd_does_not_enqueue_when_no_epr_node_exists() {
    let mut conn = establish_connection();
    clean_db(&mut conn);

    let src = insert_node(&mut conn, "direct-src", NodeKind::ClientNode);
    let dst = insert_node(&mut conn, "direct-dst", NodeKind::ClientNode);
    let _direct_link = insert_link(&mut conn, src.id, dst.id);

    let (tx, rx) = channel();
    let handler = EventLoopHandler::new(tx);

    let result = start_qkd(src.id, dst.id, &handler).await;

    assert_eq!(
        result.unwrap_err(),
        crate::error::Error::Graph(crate::error::GraphError::NoCommonEpr(src.id, dst.id))
    );
    assert!(rx.try_recv().is_err());
}
