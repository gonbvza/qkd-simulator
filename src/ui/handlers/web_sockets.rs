use std::sync::{Arc, Mutex};

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};

use crate::{
    core::event_loop::EventLoopHandler,
    models::{
        event::Event,
        event_types::{EventName, EventPayload, StoreSocketPayload},
    },
};

pub async fn web_socket_handler(
    handle: Extension<Arc<EventLoopHandler>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    println!("Socket requested");
    ws.on_failed_upgrade(|error| {
        eprintln!("WebSocket upgrade failed: {}", error);
    })
    .on_upgrade(|socket| handle_socket(handle, socket))
}

async fn handle_socket(handle: Extension<Arc<EventLoopHandler>>, mut socket: WebSocket) {
    // Send event to store socket
    let payload: StoreSocketPayload = StoreSocketPayload::new(Arc::new(Mutex::new(socket)));
    let event = Event::new_now(EventName::StoreSocket, EventPayload::StoreSocket(payload));
    handle.push_event(event).unwrap();
}
