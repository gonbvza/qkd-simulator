use crate::{
    api::{create_link_api, create_node_api, start_qkd},
    establish_connection,
    nodes::node::NodeKind,
    schema,
};
use diesel::prelude::*;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn clean_db(conn: &mut crate::PgConnection) {
    diesel::delete(schema::links::table).execute(conn).unwrap();
    diesel::delete(schema::nodes::table).execute(conn).unwrap();
}

// ── create_node_api ───────────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_node_api_client_succeeds() {
    let mut conn = establish_connection();
    clean_db(&mut conn);

    let result = create_node_api("alice".to_string(), NodeKind::ClientNode).await;

    assert!(result.is_ok());
    let node = result.unwrap();
    assert_eq!(node.name, "alice");
    assert_eq!(node.node_type, NodeKind::ClientNode.to_string());
}

#[tokio::test]
async fn test_create_node_api_epr_succeeds() {
    let mut conn = establish_connection();
    clean_db(&mut conn);

    let result = create_node_api("epr-source".to_string(), NodeKind::EprNode).await;

    assert!(result.is_ok());
    let node = result.unwrap();
    assert_eq!(node.name, "epr-source");
    assert_eq!(node.node_type, NodeKind::EprNode.to_string());
}

#[tokio::test]
async fn test_create_node_api_assigns_id() {
    let mut conn = establish_connection();
    clean_db(&mut conn);

    let node = create_node_api("bob".to_string(), NodeKind::ClientNode)
        .await
        .expect("Node creation should succeed");

    // IDs are assigned by the database and must be positive
    assert!(node.id > 0);
}

// ── create_link_api ───────────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_link_api_succeeds_with_valid_nodes() {
    let mut conn = establish_connection();
    clean_db(&mut conn);

    let src = create_node_api("src-node".to_string(), NodeKind::ClientNode)
        .await
        .expect("src node should be created");
    let dst = create_node_api("dst-node".to_string(), NodeKind::ClientNode)
        .await
        .expect("dst node should be created");

    let result = create_link_api(src.id, dst.id).await;

    assert!(result.is_ok());
    let link = result.unwrap();
    assert_eq!(link.src_id, src.id);
    assert_eq!(link.dst_id, dst.id);
    assert_eq!(link.length, 100);
    assert!((link.attenuation - 0.4).abs() < f32::EPSILON);
    assert!((link.error_rate - 0.1).abs() < f32::EPSILON);
}

#[tokio::test]
async fn test_create_link_api_fails_with_missing_src() {
    let mut conn = establish_connection();
    clean_db(&mut conn);

    let dst = create_node_api("dst-only".to_string(), NodeKind::ClientNode)
        .await
        .expect("dst node should be created");

    // Use an ID that cannot exist after a clean
    let result = create_link_api(dst.id + 999, dst.id).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_link_api_fails_with_missing_dst() {
    let mut conn = establish_connection();
    clean_db(&mut conn);

    let src = create_node_api("src-only".to_string(), NodeKind::ClientNode)
        .await
        .expect("src node should be created");

    let result = create_link_api(src.id, src.id + 999).await;

    assert!(result.is_err());
}

// ── start_qkd ─────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_start_qkd_succeeds_with_valid_topology() {
    let mut conn = establish_connection();
    clean_db(&mut conn);

    // Build the minimal topology: src --link-- epr --link-- dst
    let src = create_node_api("qkd-src".to_string(), NodeKind::ClientNode)
        .await
        .expect("src node should be created");
    let dst = create_node_api("qkd-dst".to_string(), NodeKind::ClientNode)
        .await
        .expect("dst node should be created");
    let epr = create_node_api("qkd-epr".to_string(), NodeKind::EprNode)
        .await
        .expect("epr node should be created");

    create_link_api(src.id, epr.id)
        .await
        .expect("src-epr link should be created");
    create_link_api(dst.id, epr.id)
        .await
        .expect("dst-epr link should be created");

    let result = start_qkd(src, dst).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_qkd_fails_when_epr_link_missing() {
    let mut conn = establish_connection();
    clean_db(&mut conn);

    // No links created — graph cannot resolve an EPR path
    let src = create_node_api("no-link-src".to_string(), NodeKind::ClientNode)
        .await
        .expect("src node should be created");
    let dst = create_node_api("no-link-dst".to_string(), NodeKind::ClientNode)
        .await
        .expect("dst node should be created");

    let result = start_qkd(src, dst).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_start_qkd_fails_when_no_epr_node_in_graph() {
    let mut conn = establish_connection();
    clean_db(&mut conn);

    // Two client nodes linked directly — no EPR node in the graph
    let src = create_node_api("direct-src".to_string(), NodeKind::ClientNode)
        .await
        .expect("src node should be created");
    let dst = create_node_api("direct-dst".to_string(), NodeKind::ClientNode)
        .await
        .expect("dst node should be created");

    create_link_api(src.id, dst.id)
        .await
        .expect("direct link should be created");

    let result = start_qkd(src, dst).await;

    // Graph::get_node_epr should return an error — no EPR node exists
    assert!(result.is_err());
}
