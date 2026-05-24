use crate::{
    core::{event_loop::EventLoop, settings::TIMEOUT},
    error::PairError,
    establish_connection,
    models::{measurement::Measurement, qubit_ref::QubitRefSide},
};

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

    pub fn set_measurement(&mut self, side: QubitRefSide, measurement: Measurement) {
        match side {
            QubitRefSide::Source => {
                self.src_measurement = Some(measurement);
            }
            QubitRefSide::Destination => {
                self.dst_measurement = Some(measurement);
            }
        }
    }

    pub fn get_measurement(&mut self, side: QubitRefSide) -> Result<Measurement, PairError> {
        match side {
            QubitRefSide::Source => self.dst_measurement.ok_or(PairError::NotMeasured()),
            QubitRefSide::Destination => self.src_measurement.ok_or(PairError::NotMeasured()),
        }
    }
}
