use crate::{error::PairError, models::measurement::Measurement};

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Source,
    Destination,
}

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

impl NewEntangledPair {
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
            fidelity: 1 as f32,
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

    pub fn get_measurement(&mut self, side: Side) -> Result<Measurement, PairError> {
        match side {
            Side::Source => self.dst_measurement.ok_or(PairError::NotMeasured()),
            Side::Destination => self.src_measurement.ok_or(PairError::NotMeasured()),
        }
    }
}
