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
    models::node::Node,
    models::{
        basis::Basis,
        entangled_pair::{NewEntangledPair, Side},
        event::Event,
        event_types::{EventName, EventPayload, SameBasisPayload},
        measurement::Measurement,
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
    let value = rand::random_range(0..2);
    let measurement = Measurement::new(
        node.id,
        entangled_pair.qubit_nr,
        basis,
        value,
        entangled_pair.process_id,
    );
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
    let first_measurement = entangled_pair.get_measurement(side)?;
    let mut fidelity: f32 = entangled_pair.fidelity;
    // Calculate new fidelity after distance degradation
    fidelity = calculate_new_fidelity(fidelity, distance);

    let basis: Basis = Basis::get_random_basis(side);
    let basis_diff = calculate_basis_difference(basis, first_measurement.basis);
    let prob = calculate_entanglement_prob(fidelity, basis_diff);

    // Based on the probability calculate measurement
    let rand_num = rand::random_range(0..11);
    let value: i16 = if (rand_num as f64) < (prob * 10_f64) {
        first_measurement.value
    } else {
        1 - first_measurement.value
    };

    // Instantiate measurement
    let curr_measurement = Measurement::new(
        node.id,
        entangled_pair.qubit_nr,
        basis,
        value,
        entangled_pair.process_id,
    );
    entangled_pair.set_measurement(side, curr_measurement);

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

    let payload: SameBasisPayload = SameBasisPayload::new(process.id);
    handle.push_event(Event::new_now(
        EventName::SameBasis,
        EventPayload::SameBasis(payload),
    ))?;

    Ok(())
}
