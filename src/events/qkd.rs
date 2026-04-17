use std::{collections::HashMap, sync::Arc};

use crate::{
    core::{event_loop::EventLoop, process::Process},
    database::detector::get_detector_by_id,
    error::{DetectorError, Error, NodeError, SimError},
    establish_connection, get_link_arg,
    models::{
        args::EventArgs,
        detector::Detector,
        entangled_pair::EntangledPair,
        links::Link,
        qubit_ref::{QubitRef, QubitRefSide},
    },
    nodes::node::Node,
    utility::get_node_arg,
};

// Initializes a QKD session.
//
// Ensures the source, destination, and EPR nodes are free before locking them for the session.
// If any node is busy, the session is aborted. Once locked, the EPR node begins emitting
// entangled pairs for key generation via `emit_next_pair()`.
//
// # Arguments
// * `src_node` - Source node initiating the session
// * `dst_node` - Destination node receiving the session
// * `epr_node` - EPR node used for entanglement generation
// * `src_epr_link` - Link between source and EPR node
// * `dst_epr_link` - Link between destination and EPR node
pub fn handle_qkd_init(args: &HashMap<String, EventArgs>) -> Result<(), Error> {
    let mut src_node: Node = get_node_arg(args, "src_node")?;
    let mut dst_node: Node = get_node_arg(args, "dst_node")?;
    let mut epr_node: Node = get_node_arg(args, "epr_node")?;
    let src_epr_link = get_link_arg!(args, "src_epr_link");
    let dst_epr_link = get_link_arg!(args, "dst_epr_link");

    let process = Process::new()?;

    if !src_node.try_acquire(process.id)
        || !dst_node.try_acquire(process.id)
        || !epr_node.try_acquire(process.id)
    {
        return Err(Error::Node(NodeError::NodeInUse()));
    }

    for qubit_nr in 1..1024 {
        emit_pair(
            src_node.to_owned(),
            dst_node.to_owned(),
            epr_node.to_owned(),
            src_epr_link.to_owned(),
            dst_epr_link.to_owned(),
            qubit_nr,
            process.id,
        )?;
    }
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
/// **2. PhotonTransmit events:** Schedule two qubit receival by creating qubit ref
/// and sending one to each detector. Each event timestamp
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
    src_node: Node,
    dst_node: Node,
    epr_node: Node,
    src_epr_link: Link,
    dst_epr_link: Link,
    qubit_nr: i32,
    procces_id: i32,
) -> Result<(), Error> {
    // Create entangled pair
    let entangled_pair = EntangledPair::new(src_node.id, src_node.id, procces_id)?;
    let current_time = {
        let loop_pair = Arc::clone(&*EventLoop::instance());
        let (event_loop, _) = &*loop_pair;

        let mut guard = event_loop.lock().unwrap();
        guard.get_current_time()
    };
    // Create left qubit ref and event args
    let src_qubit_ref: QubitRef = QubitRef::new(entangled_pair.id, QubitRefSide::Source);
    let src_detector_args: HashMap<String, EventArgs> = HashMap::from([
        (String::from("node"), EventArgs::Node(src_node.id)),
        (
            String::from("qubit_ref"),
            EventArgs::QubitRef(src_qubit_ref),
        ),
    ]);
    // Create right qubit ref and event args
    let dst_qubit_ref: QubitRef = QubitRef::new(entangled_pair.id, QubitRefSide::Destination);
    let dst_detector_args: HashMap<String, EventArgs> = HashMap::from([
        (String::from("node"), EventArgs::Node(dst_node.id)),
        (
            String::from("qubit_ref"),
            EventArgs::QubitRef(dst_qubit_ref),
        ),
    ]);
    EventLoop::new_and_push(
        "receive_pair_event".to_string(),
        "receive_pair".to_string(),
        src_detector_args,
        current_time + (src_epr_link.propagation_delay_us() * qubit_nr as i64),
    );
    EventLoop::new_and_push(
        "receive_pair_event".to_string(),
        "receive_pair".to_string(),
        dst_detector_args,
        current_time + (dst_epr_link.propagation_delay_us() * qubit_nr as i64),
    );
    Ok(())
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
/// * `node`        - The node that receives the qubit ref
pub fn receive_pair(args: &HashMap<String, EventArgs>) -> Result<(), Error> {
    let mut conn = establish_connection();
    let node: Node = get_node_arg(args, "node")?;
    let mut detector: Detector = get_detector_by_id(&mut conn, node.detector_id)?;
    let current_time = {
        let loop_pair = Arc::clone(&*EventLoop::instance());
        let (event_loop, _) = &*loop_pair;

        let mut guard = event_loop.lock().unwrap();
        guard.get_current_time()
    };
    if detector.is_cooling(current_time) {
        return Err(DetectorError::CoolingDown(detector.id).into());
    }

    detector.set_detection_time(current_time)?;

    Ok(())
}
