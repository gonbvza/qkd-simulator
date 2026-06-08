use crate::{
    core::settings::{
        DARK_COUNT_RATE, FIDELITY_DEGRADATION, FULL_CIRCLE, HALF_CIRCLE, SECONDS_TO_MICROSECONDS,
        SECONDS_TO_MILISECONDS,
    },
    models::basis::Basis,
};
use rand::RngExt;
use rand_distr::{Distribution, Poisson};

/// Calculates the new fidelity after distance degradation by a
/// constant factor.
pub fn calculate_new_fidelity(fidelity: f32, distance: i64) -> f32 {
    let degradation = 1.0 - (distance as f32 * FIDELITY_DEGRADATION);
    fidelity * degradation
}

/// Returns the smallest angular difference in degrees between two bases,
/// normalised to the range [0, 180].
pub fn calculate_basis_difference(basis_a: Basis, basis_b: Basis) -> f64 {
    let difference: f64 = if basis_a.angle_deg() > basis_b.angle_deg() {
        basis_a.angle_deg() - basis_b.angle_deg()
    } else {
        basis_b.angle_deg() - basis_a.angle_deg()
    };
    if difference > HALF_CIRCLE {
        return FULL_CIRCLE - difference;
    }
    difference
}

/// Calculates the entanglement probability based on degraded fidelity
/// and basis difference.
///
/// Qubits measured under the same basis and high fidelity will have a
/// high probability of returning same value.
pub fn calculate_entanglement_prob(fidelity: f32, basis_diff: f64) -> f64 {
    let diff = basis_diff.to_radians().cos().powi(2);
    fidelity as f64 * diff + (1.0 - fidelity as f64) * 0.5
}

/// Function to calculate the dark count event timestamp
///
/// It uses the dark_count_rate value of the detector as the parameter
/// for a poisson distribution. As larger the dark_count_rate, lower the
/// time between false detections.
pub fn calculate_dark_count_time() -> Vec<i64> {
    let mut rng = rand::rng();

    // 1. Sample number of dark counts in 1 second
    let poisson = Poisson::new(DARK_COUNT_RATE).unwrap();
    let n_dark = poisson.sample(&mut rng) as usize;

    // 2. Generate uniform timestamps in [0, 1s)
    let timestamps: Vec<i64> = (0..n_dark)
        .map(|_| rng.random_range(0..SECONDS_TO_MILISECONDS))
        .collect();

    timestamps
}
