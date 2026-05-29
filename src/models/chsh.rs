use derive_new::new;

use crate::{
    error::SiftingError,
    models::{basis::Basis, entangled_pair::AcceptedPair},
};

/// Instance to represent a specific group of pairs for CHSH
#[derive(Debug, Clone, new, Default)]
pub struct CHSHGroup {
    #[new(default)]
    pairs: Vec<AcceptedPair>,
    #[new(value = "0.0")]
    correlation: f32,
}

/// Instance to organize pairs with different basis into the CHSH groups
#[derive(Debug, Clone, new)]
pub struct CHSH {
    #[new(default)]
    a0b0: CHSHGroup,
    #[new(default)]
    a0b1: CHSHGroup,
    #[new(default)]
    a1b0: CHSHGroup,
    #[new(default)]
    a1b1: CHSHGroup,
}

impl CHSHGroup {
    /// Function to push pair to vector
    pub fn push_pair(&mut self, pair: AcceptedPair) {
        self.pairs.push(pair);
    }

    /// Function to calculate the correlation between measurements
    pub fn calculate_correlation(&mut self) -> f32 {
        let sum: f32 = self
            .pairs
            .iter()
            .map(|p| p.src_measurement.value as f32 * p.dst_measurement.value as f32)
            .sum();
        self.correlation = sum / self.pairs.len() as f32;
        self.correlation
    }
}

impl CHSH {
    /// Function to instatiate the CHSH groups from a list of pairs
    pub fn from_pairs(pairs: Vec<AcceptedPair>) -> Result<Self, SiftingError> {
        let mut chsh = CHSH::new();
        for mut pair in pairs {
            pair.map_values();
            chsh.push_to_group(pair)?;
        }
        Ok(chsh)
    }

    /// Function to push the pair into its corresponding group
    pub fn push_to_group(&mut self, pair: AcceptedPair) -> Result<(), SiftingError> {
        match (pair.src_measurement.basis, pair.dst_measurement.basis) {
            (Basis::Deg0, Basis::Deg22_5) => Ok(self.a0b0.push_pair(pair)),
            (Basis::Deg0, Basis::DegNeg22_5) => Ok(self.a0b1.push_pair(pair)),
            (Basis::Deg45, Basis::Deg22_5) => Ok(self.a1b0.push_pair(pair)),
            (Basis::Deg45, Basis::DegNeg22_5) => Ok(self.a1b1.push_pair(pair)),
            (_, _) => Err(SiftingError::NotKnownCombination(
                pair.src_measurement.basis,
                pair.dst_measurement.basis,
            )),
        }
    }

    /// Function to calculate the CHSH value for these groups
    pub fn calculate_chsh(&mut self) -> f32 {
        (self.a0b0.calculate_correlation()
            + self.a0b1.calculate_correlation()
            + self.a1b0.calculate_correlation()
            - self.a1b1.calculate_correlation())
        .abs()
    }
}
