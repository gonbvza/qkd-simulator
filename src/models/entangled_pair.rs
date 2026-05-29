use derive_new::new;

use crate::{error::PairError, models::measurement::Measurement};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Side {
    Source,
    Destination,
}

/// Instance that simulates a pair of entangled qubits
#[derive(Debug, Clone)]
pub struct NewEntangledPair {
    pub src_id: i32,
    pub dst_id: i32,
    pub fidelity: f32,
    pub created_at: i64,
    pub src_measurement: Option<Measurement>,
    pub dst_measurement: Option<Measurement>,
    pub timeout_timestamp: i64,
    pub process_id: i32,
    pub qubit_nr: i32,
    pub accepted: bool,
}

/// Instance used to represent an accepted pair
///
/// Made to unwrap from Option<Measurement>
#[derive(Debug, Clone, new)]
pub struct AcceptedPair {
    pub src_id: i32,
    pub dst_id: i32,
    pub src_measurement: Measurement,
    pub dst_measurement: Measurement,
    pub process_id: i32,
    pub qubit_nr: i32,
}

impl NewEntangledPair {
    /// Creates the pair with some default values
    pub fn new(
        src_id: i32,
        dst_id: i32,
        process_id: i32,
        qubit_nr: i32,
        _save: bool,
        current_time: i64,
    ) -> Result<NewEntangledPair, PairError> {
        let pair = NewEntangledPair {
            src_id,
            dst_id,
            fidelity: 1_f32,
            created_at: current_time,
            src_measurement: None,
            dst_measurement: None,
            timeout_timestamp: 0,
            process_id,
            qubit_nr,
            accepted: false,
        };

        Ok(pair)
    }

    /// Sets the measurement to the corresponding side
    pub fn set_measurement(&mut self, side: Side, measurement: Measurement) {
        match side {
            Side::Source => {
                self.src_measurement = Some(measurement);
            }
            Side::Destination => {
                self.dst_measurement = Some(measurement);
            }
        }
    }

    /// Gets the opposite measurement based on the second measurement side
    pub fn get_measurement(&mut self, side: Side) -> Result<Measurement, PairError> {
        match side {
            Side::Source => self.dst_measurement.ok_or(PairError::NotMeasured()),
            Side::Destination => self.src_measurement.ok_or(PairError::NotMeasured()),
        }
    }

    /// Function to obtain the accepted pair
    pub fn map_accepted(&self) -> Result<AcceptedPair, PairError> {
        let Some(src_measurement) = self.src_measurement else {
            return Err(PairError::NotMeasured());
        };
        let Some(dst_measurement) = self.dst_measurement else {
            return Err(PairError::NotMeasured());
        };

        Ok(AcceptedPair::new(
            self.src_id,
            self.dst_id,
            src_measurement,
            dst_measurement,
            self.process_id,
            self.qubit_nr,
        ))
    }
}

impl AcceptedPair {
    /// Function to map measurement values of 0 to -1 for CHSH
    pub fn map_values(&mut self) {
        self.src_measurement.map_value();
        self.dst_measurement.map_value();
    }
}
