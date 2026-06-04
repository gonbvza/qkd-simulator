use std::sync::Arc;

use axum::{Extension, Router};
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::{core::event_loop::EventLoopHandler, ui::routes::api::api_routes};

pub async fn start_server(handle: EventLoopHandler) {
    let cors_layer = CorsLayer::permissive();
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .fallback_service(ServeDir::new("src/ui/templates").append_index_html_on_directories(true))
        .nest("/api", api_routes())
        .layer(cors_layer)
        .layer(Extension(Arc::new(handle)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
