use axum::{
    routing::{get, post},
    Router,
};

use crate::ui::handlers::state::{create_node, get_state};

pub fn api_routes() -> Router {
    let state_routes = Router::new()
        .route("/get", get(get_state))
        .route("/node/create", post(create_node));

    Router::new().nest("/state", state_routes)
}
