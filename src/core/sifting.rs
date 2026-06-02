use crate::{
    core::settings::CHSH_THRESHOLD,
    error::SiftingError,
    models::{chsh::CHSH, entangled_pair::AcceptedPair},
    utility::{is_power_of_two, parity, split_vector},
};

/// Function to calculate the ratio of measurements that were
/// the same.
///
/// This function will be used solely for logging and debuging
pub fn compare_measurement_val(src_qubits: Vec<u8>, dst_qubits: Vec<u8>) -> f32 {
    let mut same_val = 0;
    for n in 0..src_qubits.len() {
        if src_qubits[n] == dst_qubits[n] {
            same_val += 1;
        }
    }

    (same_val as f32 / src_qubits.len() as f32) * 100_f32
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

/// Function to perform the cascade error correction algorithm
///
/// It raises SiftingError::BadLength if the key lenght is not a multiple of 2. This ensures correct
/// splitting of the vector
pub fn cascade(mut src_qubits: Vec<u8>, mut dst_qubits: Vec<u8>) -> Result<Vec<u8>, SiftingError> {
    let mut n = 1;
    // Base case
    if src_qubits.len() == 1 {
        // Split qubit of the src vector
        return Ok(vec![1 - src_qubits[0]]);
    }

    // Ensure vectors are multiple of 2
    if !is_power_of_two(src_qubits.len()) || !is_power_of_two(dst_qubits.len()) {
        return Err(SiftingError::BadLength);
    }

    loop {
        let blocks = 2_usize.pow(n);

        let mut src_blocks = split_vector(src_qubits, blocks);
        let dst_blocks = split_vector(dst_qubits, blocks);

        for n in 0..blocks {
            if parity(&src_blocks[n]) != parity(&dst_blocks[n]) {
                // Block with different parity, recurse and solve
                src_blocks[n] = cascade(src_blocks[n].clone(), src_blocks[n].clone())?;
            }
        }

        if src_blocks == dst_blocks {
            // Same as hashing and sending in actual cascade
            return Ok(src_blocks.into_iter().flatten().collect::<Vec<u8>>());
        }

        src_qubits = src_blocks.into_iter().flatten().collect::<Vec<u8>>();
        dst_qubits = dst_blocks.into_iter().flatten().collect::<Vec<u8>>();

        n += 1;
    }
}
