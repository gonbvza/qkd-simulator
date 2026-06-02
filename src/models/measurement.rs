use derive_new::new;

use crate::{error::PairError, models::basis::Basis};

/// Instance representing a measurement made by a node on a
/// single qubit of the pair
#[derive(Debug, Clone, Copy, PartialEq, new)]
pub struct Measurement {
    pub node_id: i32,
    pub process_id: i32,
    pub qubit_nr: i32,
    #[new(value = "false")]
    pub accepted: bool,
    #[new(value = "None")]
    pub mallory_value: Option<MalloryValue>,
    #[new(value = "None")]
    pub client_value: Option<ClientValue>,
}

/// Instance representing an accepted measurement. Only needs
/// the value measured by the client
#[derive(Debug, Clone, Copy, PartialEq, new)]
pub struct AcceptedMeasurement {
    pub basis: Basis,
    pub value: i16,
}

/// Instance representing a measurement made by a node on a
/// single qubit of the pair
#[derive(Debug, Clone, Copy, PartialEq, new)]
pub struct MalloryValue {
    pub basis: Basis,
    pub value: i16,
}

/// Instance representing a measurement made by a node on a
/// single qubit of the pair
#[derive(Debug, Clone, Copy, PartialEq, new)]
pub struct ClientValue {
    pub basis: Basis,
    pub value: i16,
}

impl Measurement {
    /// Set the measurement as accepted
    ///
    /// An accepted measurement means that both qubits
    /// were received and measured correctly
    pub fn set_accepted(&mut self) {
        self.accepted = true;
    }

    /// Gets the measurement value
    ///
    /// Mallory value has priority over client value
    pub fn get_value(&self) -> Result<i16, PairError> {
        // Check mallory value first
        if let Some(mallory_value) = self.mallory_value {
            return Ok(mallory_value.value);
        };

        // Check client value second
        if let Some(client_value) = self.client_value {
            return Ok(client_value.value);
        };

        Err(PairError::NotMeasured())
    }

    /// Gets the measurement value
    ///
    /// Mallory value has priority over client value
    pub fn get_basis(&self) -> Result<Basis, PairError> {
        // Check mallory value first
        if let Some(mallory_value) = self.mallory_value {
            return Ok(mallory_value.basis);
        };

        // Check client value second
        if let Some(client_value) = self.client_value {
            return Ok(client_value.basis);
        };

        Err(PairError::NotMeasured())
    }
}

impl AcceptedMeasurement {
    /// Function to map values of measurement from 0 to -1 for CHSH
    pub fn map_value(&mut self) {
        if self.value == 0 {
            self.value = -1;
        }
    }
}
