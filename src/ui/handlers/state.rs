use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::ui::domain::state_dtos::{LinkDto, NodeDto, StateDto};

pub enum ApiResponse {
    Ok,
    Created,
    JsonData(Vec<StateDto>),
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
    ApiResponse::JsonData(vec![StateDto {
        nodes: vec![
            NodeDto::new(0, "client".to_string()),
            NodeDto::new(1, "client".to_string()),
        ],
        links: vec![LinkDto::new(0, 1, false)],
    }])
}
