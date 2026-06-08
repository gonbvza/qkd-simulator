use crate::{
    core::{
        dark_count::schedule_dark_counts,
        event_loop::EventLoopHandler,
        mallory::mallory_measure_qubit,
        measurement::measure_qubit,
        pairs::{emit_pair, schedule_pair_event, schedule_pair_timeout},
        settings::QUBIT_AMOUNT,
        state::{PairKey, SimulationState},
    },
    error::{DetectorError, Error, LinkError, NodeError, PairError, SimError},
    models::{
        detector::Detector, entangled_pair::Side, event_types::EventPayload, process::Process,
    },
};
use std::cmp::max;

/// Initialises a QKD session between two nodes.
///
/// Locks the source and destination nodes for the session, then emits
/// all entangled pairs via the EPR link. Returns [`NodeError::NodeInUse`]
/// if either node is already occupied.
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

    let (nodes, links, pairs, detectors) = state.split_nodes_links_pairs_detector_mut();

    let [src_node, dst_node] = nodes
        .get_disjoint_mut([&args.src_node_id, &args.dst_node_id])
        .map(|item| item.ok_or(Error::Node(NodeError::NodeNotFound(0))));

    let src_node = src_node?;
    let dst_node = dst_node?;

    let src_epr_link = links
        .get(&args.src_epr_link_id)
        .ok_or_else(|| Error::Sim(SimError::MissingArgument("dst_node_id".to_string())))?;
    let dst_epr_link = links
        .get(&args.dst_epr_link_id)
        .ok_or_else(|| Error::Sim(SimError::MissingArgument("dst_node_id".to_string())))?;

    if !src_node.try_acquire(process_id) || !dst_node.try_acquire(process_id) {
        return Err(Error::Node(NodeError::NodeInUse()));
    }

    // Schedule dark count events
    let src_detector = detectors
        .get(&src_node.detector_id)
        .ok_or(DetectorError::NotFound(src_node.detector_id))?;
    let dst_detector = detectors
        .get(&dst_node.detector_id)
        .ok_or(DetectorError::NotFound(dst_node.detector_id))?;

    schedule_dark_counts(src_detector, current_time, handle)?;
    schedule_dark_counts(dst_detector, current_time, handle)?;

    // Remove pair array and change pair_hm name
    for qubit_nr in 1..QUBIT_AMOUNT {
        let pair_key: PairKey = PairKey {
            qubit_nr,
            process_id,
        };
        let pair = emit_pair(
            src_node.id,
            dst_node.id,
            src_epr_link.to_owned(),
            dst_epr_link.to_owned(),
            pair_key,
            current_time,
            handle,
        )?;
        pairs.insert((pair.process_id, pair.qubit_nr), pair);
    }

    println!(
        "Starting process {} at timestamp {}",
        process_id, current_time
    );
    Ok(())
}

/// Simulates a qubit arriving at a detector.
///
/// Checks whether the detector is still cooling down and skips the measurement
/// if so. Otherwise records the detection time and delegates to [`measure_qubit`].
pub fn receive_pair(
    payload: EventPayload,
    current_time: i64,
    state: &mut SimulationState,
    handle: &EventLoopHandler,
) -> Result<(), Error> {
    let (detectors, nodes, links) = state.split_detectors_nodes_links_mut();
    let EventPayload::ReceivePair(args) = payload else {
        return Err(Error::WrongArgs());
    };
    let (node, distance, is_secure, detector) = {
        let node = nodes
            .get_mut(&args.node_id)
            .ok_or(NodeError::NodeNotFound(args.node_id.to_owned()))?;
        let link = links
            .get(&args.link_id)
            .ok_or(NodeError::NodeNotFound(args.node_id.to_owned()))?;

        let detector: &mut Detector = detectors
            .get_mut(&node.detector_id)
            .ok_or(DetectorError::NotFound(node.detector_id))?;

        // Fix cloning this node as you cant change it
        (node.to_owned(), link.length, link.is_secure, detector)
    };

    if detector.is_cooling(current_time) {
        // Cooling should not rais an error, just stop
        println!("COOLING");
        return Ok(());
    };

    detector.set_detection_time(current_time)?;

    if !is_secure {
        // Channel is not secure, mallory should measure first
        mallory_measure_qubit(
            args.process_id,
            args.qubit_nr,
            args.side,
            // TODO: FIX cloning this node
            node.clone(),
            distance,
            state,
            handle,
        )?;
    }

    measure_qubit(
        args.process_id,
        args.qubit_nr,
        args.side,
        node,
        distance,
        state,
        handle,
    )?;

    Ok(())
}

/// Function to resend a pair if this one was dropped
///
/// If the pair was accepted, then it does nothing
pub fn pair_timeout(
    payload: EventPayload,
    current_time: i64,
    state: &mut SimulationState,
    handle: &EventLoopHandler,
) -> Result<(), Error> {
    let EventPayload::PairTimeout(args) = payload else {
        return Err(Error::WrongArgs());
    };

    let (_, links, pairs, _) = state.split_nodes_links_pairs_detector_mut();

    let Some(pair) = pairs.get(&(args.pair_key.process_id, args.pair_key.qubit_nr)) else {
        return Err(PairError::PairNotFound(args.pair_key.process_id).into());
    };

    // Check if pair was accepted
    if pair.accepted {
        return Ok(());
    }

    // Resending
    let src_link = links
        .get(&args.src_link)
        .ok_or(LinkError::NotFound(args.src_link))?;
    let dst_link = links
        .get(&args.dst_link)
        .ok_or(LinkError::NotFound(args.dst_link))?;

    let src_pair_ts = schedule_pair_event(
        pair.src_id,
        Side::Source,
        &src_link,
        &args.pair_key,
        current_time,
        handle,
    )?;
    let dst_pair_ts = schedule_pair_event(
        pair.dst_id,
        Side::Destination,
        &dst_link,
        &args.pair_key,
        current_time,
        handle,
    )?;

    schedule_pair_timeout(
        &src_link,
        &dst_link,
        args.pair_key,
        max(src_pair_ts, dst_pair_ts),
        handle,
    )?;
    Ok(())
}
