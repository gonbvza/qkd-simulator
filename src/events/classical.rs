use crate::{
    core::{
        event_loop::EventLoopHandler,
        sifting::{cascade, perform_chsh},
        state::SimulationState,
    },
    error::Error,
    models::{basis::Basis, entangled_pair::AcceptedPair, event_types::EventPayload},
};

pub fn post_process_key(
    payload: EventPayload,
    _current_time: i64,
    state: &mut SimulationState,
    _handle: &EventLoopHandler,
) -> Result<(), Error> {
    let EventPayload::PostProcess(args) = payload else {
        return Err(Error::WrongArgs());
    };

    let acc_pairs = state.get_accepted_measurements(args.process_id);
    let mut same_basis: Vec<AcceptedPair> = Vec::new();
    let mut diff_basis: Vec<AcceptedPair> = Vec::new();

    for pair in acc_pairs.clone() {
        let accepted = pair.map_accepted()?;
        if accepted.src_measurement.basis == accepted.dst_measurement.basis {
            same_basis.push(accepted);
            continue;
        }

        if accepted.src_measurement.basis == Basis::Deg90
            || accepted.dst_measurement.basis == Basis::Deg90
        {
            continue;
        }

        diff_basis.push(accepted);
    }

    // Calculate CHSH
    perform_chsh(diff_basis, args.process_id)?;

    // CHSH correct, correct key error
    let (src_qubits, dst_qubits) = AcceptedPair::get_qubits(same_basis);
    cascade(src_qubits, dst_qubits)?;

    Ok(())
}
