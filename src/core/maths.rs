use crate::{
    core::settings::{FIDELITY_DEGRADATION, FULL_CIRCLE, HALF_CIRCLE},
    models::basis::Basis,
};

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
