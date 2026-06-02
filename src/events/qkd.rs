use crate::{
    core::{
        event_loop::EventLoopHandler,
        mallory::mallory_measure_qubit,
        measurement::measure_qubit,
        pairs::emit_pair,
        process::Process,
        settings::QUBIT_AMOUNT,
        state::{PairKey, SimulationState},
    },
    error::{DetectorError, Error, NodeError, SimError},
    models::{detector::Detector, event_types::EventPayload},
};

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

    println!("Starting process {}", process_id);

    let (nodes, links, pairs) = state.split_nodes_links_pairs_mut();

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
    let (node, distance, is_secure) = {
        let node = nodes
            .get_mut(&args.node_id)
            .ok_or(NodeError::NodeNotFound(args.node_id.to_owned()))?;
        let link = links
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
        // Fix cloning this node as you cant change it
        (node.to_owned(), link.length, link.is_secure)
    };

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
