use std::sync::Arc;

use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};

use crate::{
    core::{event_loop::EventLoop, settings::TIMEOUT},
    database::entangled_pair::{
        change_dst_measurement, change_src_measurement, create_entangled_pair,
    },
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

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::entangled_pair)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EntangledPair {
    pub id: i32,
    pub src_id: i32,
    pub dst_id: i32,
    pub fidelity: f32,
    pub created_at: i64,
    pub src_measured: Option<i16>,
    pub dst_measured: Option<i16>,
    pub timeout_timestamp: i64,
    pub process_id: i32,
    pub qubit_nr: i32,
}

impl NewEntangledPair {
    pub fn new(
        src_id: i32,
        dst_id: i32,
        process_id: i32,
        qubit_nr: i32,
        save: bool,
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
        if save {
            let mut conn = establish_connection();
            println!("SAVING");
            create_entangled_pair(
                &mut conn,
                src_id,
                dst_id,
                current_time,
                current_time + TIMEOUT.clone(),
                process_id,
                qubit_nr,
            )?;
        }
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
            QubitRefSide::Destination => self.dst_measurement.ok_or(PairError::NotMeasured()),
        }
    }
}
