use crate::{
    core::settings::CHSH_THRESHOLD,
    error::{Error, SiftingError},
    models::{chsh::CHSH, entangled_pair::AcceptedPair},
};

/// Function to calculate the ratio of measurements that were
/// the same.
///
/// This function will be used solely for logging and debuging
pub fn compare_measurement_val(pairs: Vec<AcceptedPair>) -> Result<f32, Error> {
    let mut same_val = 0;
    for pair in pairs.clone() {
        if pair.src_measurement.value == pair.dst_measurement.value {
            same_val += 1;
        }
    }

    Ok((same_val as f32 / pairs.len() as f32) * 100_f32)
}

/// Function to perform CHSH validation logic
///
/// Function raises SiftingError::MalloryDetected if CHSH goes bellow
/// CHSH_THRESHOLD
pub fn perform_chsh(pairs: Vec<AcceptedPair>, process_id: i32) -> Result<(), SiftingError> {
    // Generate CHSH
    let mut chsh = CHSH::from_pairs(pairs)?;
    // Calculate CHSH
    let chsh_val = chsh.calculate_chsh();
    // Raise error
    if chsh_val < CHSH_THRESHOLD {
        return Err(SiftingError::MalloryDetected(chsh_val, process_id));
    }
    Ok(())
}
