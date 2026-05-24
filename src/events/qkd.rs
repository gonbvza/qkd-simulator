use crate::{
    core::{
        event_loop::EventLoopHandler, measurement::measure_qubit, process::Process,
        settings::QUBIT_AMOUNT, state::SimulationState,
    },
    error::{DetectorError, Error, NodeError, SimError},
    models::node::Node,
    models::{
        detector::Detector,
        entangled_pair::{NewEntangledPair, Side},
        event::Event,
        event_types::{EventName, EventPayload, ReceivePairPayload},
        links::Link,
    },
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
    payload: EventPayload,
    current_time: i64,
    state: &mut SimulationState,
    handle: &EventLoopHandler,
) -> Result<(), Error> {
    let EventPayload::HandleQkdInit(args) = payload else {
        return Err(Error::WrongArgs());
    };

    let process = Process::new(current_time);
    let process_id = state.push_process(process);

    let (nodes, links, pairs) = (&mut state.nodes, &mut state.links, &mut state.pairs);

    let mut src_node: Node = nodes
        .get(&args.src_node_id)
        .cloned()
        .ok_or_else(|| Error::Sim(SimError::MissingArgument("src_node_id".to_string())))?;
    let mut dst_node: Node = nodes
        .get(&args.dst_node_id)
        .cloned()
        .ok_or_else(|| Error::Sim(SimError::MissingArgument("dst_node_id".to_string())))?;
    let src_epr_link = links
        .get(&args.src_epr_link_id)
        .ok_or_else(|| Error::Sim(SimError::MissingArgument("dst_node_id".to_string())))?;
    let dst_epr_link = links
        .get(&args.dst_epr_link_id)
        .ok_or_else(|| Error::Sim(SimError::MissingArgument("dst_node_id".to_string())))?;

    if !src_node.try_acquire(process_id) || !dst_node.try_acquire(process_id) {
        return Err(Error::Node(NodeError::NodeInUse()));
    }

    // Remove pair array and change pair_hm name
    for qubit_nr in 1..QUBIT_AMOUNT {
        println!("Sending pair {}", qubit_nr);
        let pair = emit_pair(
            src_node.id,
            dst_node.id,
            src_epr_link.to_owned(),
            dst_epr_link.to_owned(),
            qubit_nr,
            process_id,
            current_time,
            handle,
        )?;
        pairs.insert((pair.process_id, pair.qubit_nr), pair);
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
    src_node_id: i32,
    dst_node_id: i32,
    src_epr_link: Link,
    dst_epr_link: Link,
    qubit_nr: i32,
    process_id: i32,
    current_time: i64,
    handle: &EventLoopHandler,
) -> Result<NewEntangledPair, Error> {
    // Create entangled pair
    let entangled_pair = NewEntangledPair::new(
        src_node_id,
        src_node_id,
        process_id,
        qubit_nr,
        false,
        current_time,
    )?;
    let src_detector_payload: ReceivePairPayload = ReceivePairPayload::new(
        src_node_id,
        Side::Source,
        qubit_nr,
        process_id,
        src_epr_link.id,
    );

    let dst_detector_payload: ReceivePairPayload = ReceivePairPayload::new(
        dst_node_id,
        Side::Destination,
        qubit_nr,
        process_id,
        dst_epr_link.id,
    );

    let _ = handle.push_event(Event::new_at(
        EventName::ReceivePair,
        EventPayload::ReceivePair(src_detector_payload),
        current_time + (src_epr_link.propagation_delay_us() * qubit_nr as i64),
    ))?;
    let _ = handle.push_event(Event::new_at(
        EventName::ReceivePair,
        EventPayload::ReceivePair(dst_detector_payload),
        current_time + (dst_epr_link.propagation_delay_us() * qubit_nr as i64),
    ))?;
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
    payload: EventPayload,
    current_time: i64,
    state: &mut SimulationState,
    handle: &EventLoopHandler,
) -> Result<(), Error> {
    let (detectors, nodes, links) = (&mut state.detectors, &mut state.nodes, &mut state.links);
    let EventPayload::ReceivePair(args) = payload else {
        return Err(Error::WrongArgs());
    };
    let node = nodes
        .get_mut(&args.node_id)
        .ok_or(NodeError::NodeNotFound(args.node_id.to_owned()))?;
    let link: &Link = links
        .get(&args.link_id)
        .ok_or(NodeError::NodeNotFound(args.node_id.to_owned()))?;

    let detector: &mut Detector = detectors
        .get_mut(&node.detector_id)
        .ok_or(DetectorError::NotFound(node.detector_id))?;

    if detector.is_cooling(current_time) {
        println!("Cooling down, skipped");
        return Err(DetectorError::CoolingDown(detector.id).into());
    }

    detector.set_detection_time(current_time)?;

    measure_qubit(
        args.process_id,
        args.qubit_nr,
        args.side,
        node.to_owned(),
        link.length,
        state,
        handle,
    )?;

    Ok(())
}
