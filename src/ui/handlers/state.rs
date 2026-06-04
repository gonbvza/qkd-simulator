use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};

use crate::{
    api::{create_link_api, create_node_api},
    core::event_loop::EventLoopHandler,
    database::{link::get_all_links, nodes::get_all_nodes},
    establish_connection,
    schema::entangled_pair::dst_id,
    ui::domain::state_dtos::{CreateLinkDto, CreateNodeDto, LinkDto, NodeDto, StateDto},
};

pub enum ApiResponse {
    Ok,
    Created,
    JsonData(StateDto),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::Ok => StatusCode::OK.into_response(),
            Self::Created => StatusCode::CREATED.into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

pub async fn get_state() -> ApiResponse {
    let mut conn = establish_connection();
    // Get from database
    let nodes: Vec<NodeDto> = get_all_nodes(&mut conn)
        .unwrap()
        .iter()
        .map(|node| NodeDto::new(node.id, node.node_type.parse().unwrap()))
        .collect();
    let links: Vec<LinkDto> = get_all_links(&mut conn)
        .unwrap()
        .iter()
        .map(|link| LinkDto::new(link.src_id, link.dst_id, link.length, link.is_secure))
        .collect();

    ApiResponse::JsonData(StateDto {
        nodes: nodes,
        links: links,
    })
}

pub async fn create_node(
    handle: Extension<Arc<EventLoopHandler>>,
    Json(payload): Json<CreateNodeDto>,
) -> ApiResponse {
    // Give responsability to API layer
    create_node_api(payload.name, payload.node_type.parse().unwrap(), &handle)
        .await
        .unwrap();
    ApiResponse::Ok
}

pub async fn create_link(
    handle: Extension<Arc<EventLoopHandler>>,
    Json(payload): Json<CreateLinkDto>,
) -> ApiResponse {
    // Give responsability to API layer
    create_link_api(
        payload.src_id,
        payload.dst_id,
        payload.distance,
        payload.is_secure,
        &handle,
    )
    .await
    .unwrap();
    ApiResponse::Ok
}
