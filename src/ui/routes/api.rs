use axum::{routing::get, Router};

use crate::ui::handlers::state::get_state;

pub fn api_routes() -> Router {
    let state_routes = Router::new().route("/", get(get_state));

    Router::new().nest("/state", state_routes)
}
