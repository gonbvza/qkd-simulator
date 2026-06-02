use std::collections::HashMap;

use crate::{
    core::{
        event_loop::EventLoopHandler,
        maths::{calculate_basis_difference, calculate_entanglement_prob, calculate_new_fidelity},
        process::Process,
        settings::QUBIT_AMOUNT,
        state::SimulationState,
    },
    error::PairError,
    models::{
        basis::Basis,
        entangled_pair::{NewEntangledPair, Side},
        event::Event,
        event_types::{EventName, EventPayload, PostProcessPayload},
        measurement::{ClientValue, Measurement},
        node::Node,
    },
    utility::is_first,
};

/// Entry point for the measurement process.
///
/// Checks if this qubit is the first or second measurement
/// depending on the order logic will differ
pub fn measure_qubit(
    process_id: i32,
    qubit_nr: i32,
    side: Side,
    node: Node,
    distance: i64,
    state: &mut SimulationState,
    handle: &EventLoopHandler,
) -> Result<(), PairError> {
    let (pairs, processes, nodes) = state.split_pairs_processes_nodes_mut();
    let entangled_pair = pairs
        .get_mut(&(process_id, qubit_nr))
        .ok_or(PairError::PairNotFound(qubit_nr))?;
    let process = processes
        .get_mut(&process_id)
        .ok_or(PairError::NotMeasured())?;
    match is_first(entangled_pair, side)? {
        true => first_measurement(entangled_pair, side, node)?,
        false => second_measurement(entangled_pair, side, node, distance, process, nodes, handle)?,
    };
    Ok(())
}

/// Logic for the first measurement.
///
/// The first measurement does not depend on anything
/// so its free to chose the basis and value it measures
pub fn first_measurement(
    entangled_pair: &mut NewEntangledPair,
    side: Side,
    node: Node,
) -> Result<(), PairError> {
    let basis: Basis = Basis::get_random_basis(side);
    let value = if rand::random::<bool>() { 0 } else { 1 };

    // Get or create measurement
    let mut measurement: Measurement =
        if let Some(measurement) = entangled_pair.get_measurement(side) {
            // Mallory measured qubit first so measurement exists
            measurement
        } else {
            Measurement::new(node.id, entangled_pair.qubit_nr, entangled_pair.process_id)
        };

    measurement.client_value = Some(ClientValue::new(basis, value));
    entangled_pair.set_measurement(side, measurement);
    Ok(())
}

/// Logic for the second measurement.
///
/// Calculates the measured value based on entanglement probability,
/// which depends on the degraded fidelity and basis difference with
/// the first measurement. Once all pairs are accepted, releases both
/// nodes and triggers the classical sifting phase.
pub fn second_measurement(
    entangled_pair: &mut NewEntangledPair,
    side: Side,
    mut node: Node,
    distance: i64,
    process: &mut Process,
    nodes: &mut HashMap<i32, Node>,
    handle: &EventLoopHandler,
) -> Result<(), PairError> {
    let first_measurement = entangled_pair
        .get_measurement(side.opposite())
        .ok_or(PairError::NotMeasured())?;
    let mut fidelity: f32 = entangled_pair.fidelity;
    // Calculate new fidelity after distance degradation
    fidelity = calculate_new_fidelity(fidelity, distance);

    let basis: Basis = Basis::get_random_basis(side);
    let basis_diff = calculate_basis_difference(basis, first_measurement.get_basis()?);
    let prob_same = calculate_entanglement_prob(fidelity, basis_diff);

    // Based on the probability calculate measurement
    let r: f64 = rand::random::<f64>();
    let value = if r < prob_same {
        // correlated outcomes
        first_measurement.get_value()?
    } else {
        // anti-correlated outcomes
        1 - first_measurement.get_value()?
    };

    // Instantiate measurement
    let mut measurement: Measurement =
        if let Some(measurement) = entangled_pair.get_measurement(side) {
            // Mallory measured qubit first so measurement exists
            measurement
        } else {
            Measurement::new(node.id, entangled_pair.qubit_nr, entangled_pair.process_id)
        };
    measurement.client_value = Some(ClientValue::new(basis, value));
    entangled_pair.set_measurement(side, measurement);

    if entangled_pair.src_measurement.is_some()
        && entangled_pair.dst_measurement.is_some()
        && !entangled_pair.accepted
    {
        entangled_pair.accepted = true;
        process.accepted_pairs += 1;
    }

    // Check if all pairs have been accepted
    if process.accepted_pairs <= QUBIT_AMOUNT - 2 {
        return Ok(());
    }

    // Start classical sifting
    node.release(entangled_pair.process_id)?;
    let first_node = nodes
        .get_mut(&first_measurement.node_id)
        .ok_or(PairError::PairNotFound(first_measurement.node_id))?;
    first_node.release(entangled_pair.process_id)?;

    let payload: PostProcessPayload = PostProcessPayload::new(process.id);
    handle.push_event(Event::new_now(
        EventName::PostProcess,
        EventPayload::PostProcess(payload),
    ))?;

    Ok(())
}
