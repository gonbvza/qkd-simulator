use crate::{
    core::settings::{FIDELITY_DEGRADATION, FULL_CIRCLE, HALF_CIRCLE},
    models::basis::Basis,
};

pub fn calculate_new_fidelity(fidelity: f32, distance: i64) -> f32 {
    let degradation = 1.0 - (distance as f32 * FIDELITY_DEGRADATION);
    fidelity * degradation
}

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

pub fn calculate_entanglement_prob(fidelity: f32, basis_diff: f64) -> f64 {
    let depolarizing_noise = (1 as f64 - fidelity as f64) * 0.5;
    // `basis_diff` is expressed in degrees; convert to radians for `cos`.
    let angle = basis_diff.to_radians().cos();
    let prob = (fidelity as f64 * angle.powi(2)) + depolarizing_noise;
    return prob;
}
