use crate::{
    core::{
        maths::{calculate_basis_difference, calculate_entanglement_prob, calculate_new_fidelity},
        process::Process,
        settings::KEY_LENGTH,
        state::SimulationState,
    },
    database::{entangled_pair::get_pair, nodes::get_node_by_id},
    error::{PairError, ProcessError},
    establish_connection,
    models::{
        basis::Basis,
        entangled_pair::{self, NewEntangledPair},
        measurement::Measurement,
        qubit_ref::{QubitRef, QubitRefSide},
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
) -> Result<(), PairError> {
    let (pairs, processes) = (&mut state.pairs, &mut state.processes);
    let entangled_pair = pairs
        .get_mut(&(process_id, qubit_nr))
        .ok_or(PairError::PairNotFound(qubit_nr))?;
    let process = processes
        .get_mut(&process_id)
        .ok_or(PairError::NotMeasured())?;
    match is_first(entangled_pair, side)? {
        true => first_measurement(entangled_pair, side, node)?,
        false => second_measurement(entangled_pair, side, node, distance, process)?,
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
    // TODO: CHANGE THIS TO A VALID PROCESS ERROR
    if entangled_pair.src_measurement.is_some()
        && entangled_pair.dst_measurement.is_some()
        && !entangled_pair.accepted
    {
        entangled_pair.accepted = true;
        process.accepted_pairs += 1;
    }
    // 9. Check if process has finished
    if process.accepted_pairs != KEY_LENGTH {
        return Ok(());
    }
    // 10. If so, start classical sifting
    node.release(entangled_pair.process_id)?;
    // TODO: TODO: REMOVE THIS CONNECTION
    let mut conn = establish_connection();
    let mut first_node = get_node_by_id(&mut conn, first_measurement.node_id)?;
    first_node.release(entangled_pair.process_id)?;

    println!("Finished qkd, starting classical sifting");
    Ok(())
}
