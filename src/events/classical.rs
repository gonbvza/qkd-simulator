use crate::{
    core::{
        event_loop::EventLoopHandler,
        settings::KEY_LENGTH,
        sifting::{cascade, perform_chsh},
        state::SimulationState,
    },
    error::{Error, ProcessError},
    models::{
        basis::Basis, entangled_pair::AcceptedPair, event_types::EventPayload, process::Process,
    },
    utility::bits_to_bytes,
};

pub fn post_process_key(
    payload: EventPayload,
    current_time: i64,
    state: &mut SimulationState,
    _handle: &EventLoopHandler,
) -> Result<(), Error> {
    let EventPayload::PostProcess(args) = payload else {
        return Err(Error::WrongArgs());
    };

    let Some(process) = state.get_process(args.process_id) else {
        return Err(ProcessError::NotFound(args.process_id).into());
    };

    if !process.key.is_none() {
        // Key was already calculated
        return Ok(());
    }

    let acc_pairs = state.get_accepted_measurements(args.process_id);
    let mut same_basis_pairs = Vec::new();
    let mut diff_basis_pairs = Vec::new();

    for pair in acc_pairs {
        let accepted = pair.map_accepted()?;

        let is_same_basis = accepted.src_measurement.basis == accepted.dst_measurement.basis;

        if is_same_basis {
            if same_basis_pairs.len() < KEY_LENGTH {
                same_basis_pairs.push(accepted);
            }
            continue;
        }

        if accepted.src_measurement.basis == Basis::Deg90
            || accepted.dst_measurement.basis == Basis::Deg90
        {
            continue;
        }

        diff_basis_pairs.push(accepted);
    }

    // Use mismatched basis measurements to evaluate the CHSH parameter.
    perform_chsh(diff_basis_pairs, args.process_id)?;

    // If CHSH test passes, correct key discrepancies.
    let (src_qubits, dst_qubits) = AcceptedPair::get_qubits_vec(same_basis_pairs);

    // Perfom error correction with cascade
    let corrected_key = cascade(src_qubits.clone(), dst_qubits.clone())?;

    // Store key in process
    let process: &mut Process = state
        .get_process_mut(args.process_id)
        .ok_or(ProcessError::NotFound(args.process_id))?;
    process.key = Some(hex::encode(bits_to_bytes(&corrected_key)));

    println!("Process finished at {}", current_time);
    Ok(())
}
