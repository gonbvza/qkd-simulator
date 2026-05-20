use std::{collections::HashMap, sync::Arc};

use diesel::{insert_into, RunQueryDsl};

use crate::{
    core::{
        event_loop::EventLoop,
        measurement::measure_qubit,
        process::Process,
        settings::{CHUNK_SIZE, QUBIT_AMOUNT},
        state::SimulationState,
    },
    database::detector::get_detector_by_id,
    error::{DetectorError, Error, NodeError, PairError, SimError},
    establish_connection, get_link_arg, get_node_arg, get_number_arg, get_pairs_arg,
    get_qubit_ref_arg, get_side_arg,
    models::{
        args::EventArgs,
        detector::Detector,
        entangled_pair::{self, EntangledPair, NewEntangledPair},
        links::Link,
        measurement::Measurement,
        qubit_ref::{QubitRef, QubitRefSide},
    },
    nodes::node::Node,
    schema,
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
pub fn handle_qkd_init(
    args: &HashMap<String, EventArgs>,
    current_time: i64,
    state: &mut SimulationState,
) -> Result<(), Error> {
    let mut src_node: Node = get_node_arg!(args, "src_node").to_owned();
    let mut dst_node: Node = get_node_arg!(args, "dst_node").to_owned();
    let epr_node: Node = get_node_arg!(args, "epr_node").to_owned();
    let src_epr_link = get_link_arg!(args, "src_epr_link");
    let dst_epr_link = get_link_arg!(args, "dst_epr_link");

    let process = Process::new(current_time);
    let process_id = state.push_process(process);

    if !src_node.try_acquire(process_id) || !dst_node.try_acquire(process_id) {
        return Err(Error::Node(NodeError::NodeInUse()));
    }

    // Remove pair array and change pair_hm name
    for qubit_nr in 1..QUBIT_AMOUNT {
        println!("Sending pair {}", qubit_nr);
        let pair = emit_pair(
            src_node.to_owned(),
            dst_node.to_owned(),
            epr_node.to_owned(),
            src_epr_link.to_owned(),
            dst_epr_link.to_owned(),
            qubit_nr,
            process_id,
            current_time,
        )?;
        state.insert_pair(pair);
    }
    // for chunk in pairs.chunks(CHUNK_SIZE) {
    //     insert_into(schema::entangled_pair::table)
    //         .values(chunk)
    //         .execute(&mut conn)
    //         .map_err(PairError::from)?;
    // }
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
    current_time: i64,
) -> Result<NewEntangledPair, Error> {
    // Create entangled pair
    let entangled_pair = NewEntangledPair::new(
        src_node.id,
        src_node.id,
        procces_id,
        qubit_nr,
        false,
        current_time,
    )?;
    let src_detector_args: HashMap<String, EventArgs> = HashMap::from([
        (String::from("node"), EventArgs::Node(src_node)),
        (String::from("side"), EventArgs::Side(QubitRefSide::Source)),
        (String::from("qubit_nr"), EventArgs::Number(qubit_nr)),
        (String::from("procces_id"), EventArgs::Number(procces_id)),
        (String::from("link"), EventArgs::Link(src_epr_link.clone())),
    ]);
    let dst_detector_args: HashMap<String, EventArgs> = HashMap::from([
        (String::from("node"), EventArgs::Node(dst_node)),
        (
            String::from("side"),
            EventArgs::Side(QubitRefSide::Destination),
        ),
        (String::from("qubit_nr"), EventArgs::Number(qubit_nr)),
        (String::from("procces_id"), EventArgs::Number(procces_id)),
        (String::from("link"), EventArgs::Link(dst_epr_link.clone())),
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
    Ok(entangled_pair)
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
/// * `link`        - The link through which the qubit was received
pub fn receive_pair(
    args: &HashMap<String, EventArgs>,
    current_time: i64,
    state: &mut SimulationState,
) -> Result<(), Error> {
    let node: &Node = get_node_arg!(args, "node");
    let link: &Link = get_link_arg!(args, "link");
    let qubit_nr: &i32 = get_number_arg!(args, "qubit_nr");
    let procces_id: &i32 = get_number_arg!(args, "procces_id");
    let side: &QubitRefSide = get_side_arg!(args, "side");
    let detector: &mut Detector = state
        .get_detector_mut(node.detector_id)
        .ok_or(DetectorError::NotFound(node.detector_id))?;

    if detector.is_cooling(current_time) {
        println!("Cooling down, skipped");
        return Err(DetectorError::CoolingDown(detector.id).into());
    }

    detector.set_detection_time(current_time)?;

    // TODO: Think about the to_owned, maybe is good to change from macro to function and own the
    // instance
    measure_qubit(
        *procces_id,
        *qubit_nr,
        side.clone(),
        node.to_owned(),
        link.length,
        state,
    )?;

    Ok(())
}
