use std::collections::HashMap;

use crate::{
    core::{
        event_loop::EventLoopHandler,
        maths::{calculate_basis_difference, calculate_entanglement_prob, calculate_new_fidelity},
        process::Process,
        settings::{KEY_LENGTH, QUBIT_AMOUNT},
        state::SimulationState,
    },
    error::PairError,
    models::{
        basis::Basis,
        entangled_pair::NewEntangledPair,
        event::Event,
        event_types::{EventName, EventPayload, SameBasisPayload},
        measurement::Measurement,
        qubit_ref::QubitRefSide,
    },
    nodes::node::Node,
    utility::is_first,
};

pub fn measure_qubit(
    process_id: i32,
    qubit_nr: i32,
    side: QubitRefSide,
    node: Node,
    distance: i64,
    state: &mut SimulationState,
    handle: &EventLoopHandler,
) -> Result<(), PairError> {
    let (pairs, processes, nodes) = (&mut state.pairs, &mut state.processes, &mut state.nodes);
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

pub fn first_measurement(
    entangled_pair: &mut NewEntangledPair,
    side: QubitRefSide,
    node: Node,
) -> Result<(), PairError> {
    // 1. Chose random basis
    let basis: Basis = Basis::get_random_basis(side);
    // 2. Calculate random number between {0,1}
    let value = rand::random_range(0..2);
    let measurement = Measurement::new(
        node.id,
        entangled_pair.qubit_nr,
        basis,
        value,
        entangled_pair.process_id,
    );
    // 3. Store measurement in the entangled pair instance
    entangled_pair.set_measurement(side, measurement);
    Ok(())
}

pub fn second_measurement(
    entangled_pair: &mut NewEntangledPair,
    side: QubitRefSide,
    mut node: Node,
    distance: i64,
    process: &mut Process,
    nodes: &mut HashMap<i32, Node>,
    handle: &EventLoopHandler,
) -> Result<(), PairError> {
    let first_measurement = entangled_pair.get_measurement(side)?;
    // 1. Get fidelity from entangled_pair
    let mut fidelity: f32 = entangled_pair.fidelity;
    // 2. Calculate new fidelity after distance degradation
    fidelity = calculate_new_fidelity(fidelity, distance);
    // 3. Chose random basis
    let basis: Basis = Basis::get_random_basis(side);
    // 4. Calculate basis diference between both
    let basis_diff = calculate_basis_difference(basis, first_measurement.basis);
    // 5. Calculate probability of entanglement
    let prob = calculate_entanglement_prob(fidelity, basis_diff);
    // 6. Based on this probability calculate measuremen
    let rand_num = rand::random_range(0..11);
    let value: i16 = if (rand_num as f64) < (prob * 10 as f64) {
        first_measurement.value
    } else {
        1 - first_measurement.value
    };
    // 7. Store measuremtn in the vector
    let curr_measurement = Measurement::new(
        node.id,
        entangled_pair.qubit_nr,
        basis,
        value,
        entangled_pair.process_id,
    );
    entangled_pair.set_measurement(side, curr_measurement);
    // 8. Accept both measurements
    if entangled_pair.src_measurement.is_some()
        && entangled_pair.dst_measurement.is_some()
        && !entangled_pair.accepted
    {
        entangled_pair.accepted = true;
        process.accepted_pairs += 1;
    }
    // 9. Check if process has finished
    if process.accepted_pairs <= QUBIT_AMOUNT - 100 {
        return Ok(());
    }
    // 10. If so, start classical sifting
    node.release(entangled_pair.process_id)?;
    let first_node = nodes
        .get_mut(&first_measurement.node_id)
        .ok_or(PairError::PairNotFound(first_measurement.node_id))?;
    first_node.release(entangled_pair.process_id)?;

    let payload: SameBasisPayload = SameBasisPayload::new(process.id);
    handle.push_event(Event::new_now(
        EventName::SameBasis,
        EventPayload::SameBasis(payload),
    ));

    println!("Finished qkd, starting classical sifting");
    Ok(())
}
