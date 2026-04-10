use std::collections::HashMap;

use crate::{
    database::nodes::set_node_usage,
    error::{Error, NodeError, SimError},
    establish_connection,
    models::{args::EventArgs, links::Link},
    nodes::node::Node,
};

/// Executes the QKD session initialization after the classical latency delay.
///
/// This function is invoked by the event loop when a [`EventType::QkdInit`] event
/// is dequeued. It performs three steps:
///
/// 1. Availability check: Verifies that the sender, receiver, and EPR node
/// all have `in_use = false`. If any node is already occupied, the event is
/// dropped and no session is started. The caller should implement a retry
/// mechanism if required.
///
/// 2. Node locking: Marks all three nodes as `in_use = true`. Because the
/// event loop is single-threaded and processes one event at a time, this
/// check-and-lock is inherently atomic — no two sessions can race.
///
/// 3. Pair emission: Calls [`EprNode::emit_next_pair`] for each qubit
///
/// # Arguments
/// * `sender_id`   - The [`NodeId`] of the initiating client node
/// * `receiver_id` - The [`NodeId`] of the destination client node
/// * `epr_id`      - The [`NodeId`] of the EPR node for this session
pub fn handle_qkd_init(args: &HashMap<String, EventArgs>) -> Result<(), Error> {
    dbg!(args);
    Ok(())
}

/// Creates one entangled pair and schedules its transmission to both client nodes.
///
/// This function is the core pump of the QKD pipeline. It is first called by
/// [`handle_qkd_init`] and then re-invoked after each successful pair measurement
/// or timeout resolution, continuing until 1024 accepted pairs have been collected.
///
/// It performs three scheduling operations:
///
/// **1. EntangledPair creation:** Instantiates a new [`EntangledPair`] with the
/// given destination nodes and an initial fidelity. The pair is stored in the
/// repository and assigned a unique `pair_id`.
///
/// **2. PhotonTransmit events:** Schedules two [`EventType::PhotonTransmit`] events,
/// one targeting `sender_id` and one targeting `receiver_id`. Each event timestamp
/// is calculated from the link's `next_available_time` and propagation delay:
/// `t = max(current_time, link.next_available_time) + t_propagation`
///
/// **3. Timeout event:** Schedules one [`EventType::MeasurementTimeout`] event for
/// this pair at:
/// `t = max(t_send, link.next_available_time) + t_propagation + detector.cooldown_ps
///      + detector.resolution_ps + SAFETY_MARGIN_PS`
/// If this event fires before both measurements are recorded, the pair is considered
/// lost and a new call to [`EprNode::emit_next_pair`] is triggered automatically.
///
/// # Arguments
/// * `sender_id`   - The [`NodeId`] of the client node receiving the left qubit
/// * `receiver_id` - The [`NodeId`] of the client node receiving the right qubit
pub fn emit_pair(
    left_node: &mut Node,
    right_node: &mut Node,
    epr: &mut Node,
    link_a: Link,
    link_b: Link,
) {
    // Generate entangled pair
    todo!();
}

/// Simulates the receival of a qubit ref by a detector.
///
/// It will fist check if the current node is in use, if so, it will check
/// the process hash against the current on going proces hash
///
/// If node is free, it sets node usage. Then it continues with the measurement
///
/// # Arguments
/// * `qubit_ref`   - The [`qubit_ref`] of the entangled pair
pub fn receive_pair(args: &HashMap<String, EventArgs>) {
    todo!();
}
