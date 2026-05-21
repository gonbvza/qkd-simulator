use std::collections::HashMap;

use crate::{
    core::{event_loop::EventLoopHandler, state::SimulationState},
    error::{Error, PairError, SimError},
    get_number_arg,
    models::args::EventArgs,
};

pub fn same_basis(
    args: &HashMap<String, EventArgs>,
    _current_time: i64,
    state: &mut SimulationState,
    _handle: &EventLoopHandler,
) -> Result<(), Error> {
    println!("Received same bassi");
    let process_id = get_number_arg!(args, "process_id").to_owned();
    let acc_pairs = state.get_accepted_measurements(process_id);
    let mut same_basis_num = 0;

    for pair in acc_pairs.clone() {
        let Some(src_measurement) = pair.src_measurement else {
            return Err(Error::Pair(PairError::NotMeasured()))
        };
        let Some(dst_measurement) = pair.dst_measurement else {
            return Err(Error::Pair(PairError::NotMeasured()))
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
