use axum::Router;
use tower_http::services::ServeDir;

use crate::ui::routes::api::api_routes;

pub async fn start_server() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .fallback_service(ServeDir::new("src/ui/templates").append_index_html_on_directories(true))
        .nest("/api", api_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
