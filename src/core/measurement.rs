use crate::{
    core::{
        maths::{calculate_basis_difference, calculate_entanglement_prob, calculate_new_fidelity},
        settings::KEY_LENGTH,
    },
    database::{
        entangled_pair::get_pair_by_id,
        measurement::{get_accepted_measurements, get_measurement_for_pair},
        nodes::get_node_by_id,
    },
    error::PairError,
    establish_connection,
    models::{basis::Basis, measurement::Measurement, qubit_ref::QubitRef},
    nodes::node::Node,
    utility::{form_word, is_first},
};

pub fn measure_qubit(qubit_ref: QubitRef, node: Node, distance: i64) -> Result<(), PairError> {
    match is_first(qubit_ref.entangled_pair_id, qubit_ref.side)? {
        true => first_measurement(qubit_ref, node)?,
        false => second_measurement(qubit_ref, node, distance)?,
    };
    Ok(())
}

pub fn first_measurement(qubit_ref: QubitRef, node: Node) -> Result<(), PairError> {
    let mut conn = establish_connection();
    let mut entangled_pair = get_pair_by_id(&mut conn, qubit_ref.entangled_pair_id)?;
    // 1. Chose random basis
    let basis: Basis = Basis::get_random_basis(qubit_ref.side);
    // 2. Calculate random number between {0,1}
    let value = rand::random_range(0..2);
    let _ = Measurement::new(
        node.id,
        qubit_ref.entangled_pair_id,
        basis,
        value,
        entangled_pair.process_id,
    )?;
    // 3. Store measurement in the entangled pair instance
    entangled_pair.set_measurement(qubit_ref.side, value);
    Ok(())
}

pub fn second_measurement(
    qubit_ref: QubitRef,
    mut node: Node,
    distance: i64,
) -> Result<(), PairError> {
    let mut conn = establish_connection();
    let mut entangled_pair = get_pair_by_id(&mut conn, qubit_ref.entangled_pair_id)?;
    let mut first_measurement = get_measurement_for_pair(&mut conn, qubit_ref.entangled_pair_id)?;
    let mut first_node = get_node_by_id(&mut conn, first_measurement.node_id)?;
    // 1. Get fidelity from entangled_pair
    let mut fidelity: f32 = entangled_pair.fidelity;
    // 2. Calculate new fidelity after distance degradation
    fidelity = calculate_new_fidelity(fidelity, distance);
    // 3. Chose random basis
    let basis: Basis = Basis::get_random_basis(qubit_ref.side);
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
    let mut curr_measurement = Measurement::new(
        node.id,
        qubit_ref.entangled_pair_id,
        basis,
        value,
        entangled_pair.process_id,
    )?;
    entangled_pair.set_measurement(qubit_ref.side, value);
    // 8. Accept both measurements
    first_measurement.set_accepted()?;
    curr_measurement.set_accepted()?;
    // 9. Check if process has finished
    let accepted_measurements: Vec<Measurement> =
        get_accepted_measurements(&mut conn, entangled_pair.process_id, node.id)?;
    if accepted_measurements.len() != KEY_LENGTH {
        return Ok(());
    }
    // 10. If so, start classical sifting
    node.release(entangled_pair.process_id)?;
    first_node.release(entangled_pair.process_id)?;

    println!("Finished qkd, starting classical sifting");
    Ok(())
}
