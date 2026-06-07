use std::sync::{Arc, Mutex};

use crate::models::entangled_pair::Side;
use crate::models::node::NodeKind;
use axum::extract::ws::WebSocket;
use derive_new::new;

/// List of event functions the system can execute.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventName {
    TestEvent,
    CreateNode,
    CreateLink,
    HandleQkdInit,
    ReceivePair,
    MeasurementTimeout,
    PostProcess,
    StoreSocket,
}

/// Strongly-typed payloads for events. Each enum variant carries a dedicated
/// struct which expresses the exact arguments required by the handler.
#[derive(Debug, Clone)]
pub enum EventPayload {
    TestEvent(),
    CreateNode(CreateNodePayload),
    CreateLink(CreateLinkPayload),
    HandleQkdInit(HandleQkdInitPayload),
    ReceivePair(ReceivePairPayload),
    MeasurementTimeout(MeasurementTimeoutPayload),
    PostProcess(PostProcessPayload),
    StoreSocket(StoreSocketPayload),
}

/// Arguments for the `create_node` event.
#[derive(Debug, Clone, new)]
pub struct CreateNodePayload {
    pub name: String,
    pub node_type: NodeKind,
}

/// Arguments for the `create_link` event.
#[derive(Debug, Clone, new)]
pub struct CreateLinkPayload {
    pub src_id: i32,
    pub dst_id: i32,
    pub distance: i64,
    pub is_secure: bool,
}

/// Arguments for the high-level QKD initialization event.
#[derive(Debug, Clone, new)]
pub struct HandleQkdInitPayload {
    pub src_node_id: i32,
    pub dst_node_id: i32,
    pub epr_node_id: i32,
    pub src_epr_link_id: i32,
    pub dst_epr_link_id: i32,
}

/// Arguments for the `receive_pair` detector event.
#[derive(Debug, Clone, new)]
pub struct ReceivePairPayload {
    pub node_id: i32,
    pub side: Side,
    pub qubit_nr: i32,
    pub process_id: i32,
    pub link_id: i32,
}

/// Arguments for a measurement timeout event.
#[derive(Debug, Clone, new)]
pub struct MeasurementTimeoutPayload {
    pub process_id: i32,
    pub qubit_nr: i32,
}

/// Arguments for sifting / same-basis evaluation.
#[derive(Debug, Clone, new)]
pub struct PostProcessPayload {
    pub process_id: i32,
}

/// Arguments for storing socket.
#[derive(Debug, Clone, new)]
pub struct StoreSocketPayload {
    pub socket: Arc<Mutex<WebSocket>>,
}
