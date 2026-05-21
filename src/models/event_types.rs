//! Strongly-typed event function identifiers and payload structs.
//!
//! This module defines the canonical set of event kinds (`EventFn`) and
//! the corresponding typed payloads for each event.

use crate::models::node::NodeKind;
use crate::models::qubit_ref::QubitRefSide;
use derive_new::new;

/// Canonical list of event functions the system can execute.
///
/// Keep this enum small and stable; add new variants only when introducing
/// new runtime behaviors. Use these variants for dispatching instead of raw
/// strings.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventName {
    CreateNode,
    CreateLink,
    HandleQkdInit,
    ReceivePair,
    MeasurementTimeout,
    SameBasis,
}

/// Strongly-typed payloads for events. Each enum variant carries a dedicated
/// struct which expresses the exact arguments required by the handler.
#[derive(Debug, Clone)]
pub enum EventPayload {
    CreateNode(CreateNodePayload),
    CreateLink(CreateLinkPayload),
    HandleQkdInit(HandleQkdInitPayload),
    ReceivePair(ReceivePairPayload),
    MeasurementTimeout(MeasurementTimeoutPayload),
    SameBasis(SameBasisPayload),
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
    pub side: QubitRefSide,
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
pub struct SameBasisPayload {
    pub process_id: i32,
}
