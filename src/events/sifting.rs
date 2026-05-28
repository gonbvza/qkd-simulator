use crate::{
    core::{event_loop::EventLoopHandler, state::SimulationState},
    error::{Error, PairError},
    models::event_types::EventPayload,
};

pub fn same_basis(
    payload: EventPayload,
    _current_time: i64,
    state: &mut SimulationState,
    _handle: &EventLoopHandler,
) -> Result<(), Error> {
    let EventPayload::SameBasis(args) = payload else {
        return Err(Error::WrongArgs());
    };

    let acc_pairs = state.get_accepted_measurements(args.process_id);
    let mut same_basis_num = 0;

    for pair in acc_pairs.clone() {
        let Some(src_measurement) = pair.src_measurement else {
            return Err(Error::Pair(PairError::NotMeasured()));
        };
        let Some(dst_measurement) = pair.dst_measurement else {
            return Err(Error::Pair(PairError::NotMeasured()));
        };

        if src_measurement.basis == dst_measurement.basis {
            same_basis_num += 1;
            println!(
                "Src: {}, Dst: {}",
                src_measurement.value, dst_measurement.value
            );
        }
    }

    println!("Number of acepted pairs {}", acc_pairs.len());
    println!("Number of acepted with same basis were {}", same_basis_num);
    Ok(())
}
