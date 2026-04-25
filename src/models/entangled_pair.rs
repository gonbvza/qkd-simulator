use std::sync::Arc;

use diesel::{prelude::Queryable, Selectable};

use crate::{
    core::{event_loop::EventLoop, settings::TIMEOUT},
    database::entangled_pair::{
        change_dst_measurement, change_src_measurement, create_entangled_pair,
    },
    error::PairError,
    establish_connection,
    models::qubit_ref::QubitRefSide,
};

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

impl EntangledPair {
    pub fn new(
        src_id: i32,
        dst_id: i32,
        process_id: i32,
        qubit_nr: i32,
    ) -> Result<EntangledPair, PairError> {
        let mut conn = establish_connection();
        let current_time = {
            let loop_pair = Arc::clone(&*EventLoop::instance());
            let (event_loop, _) = &*loop_pair;

            let mut guard = event_loop.lock().unwrap();
            guard.get_current_time()
        };
        create_entangled_pair(
            &mut conn,
            src_id,
            dst_id,
            current_time,
            current_time + TIMEOUT.clone(),
            process_id,
            qubit_nr,
        )
    }

    pub fn set_measurement(&mut self, side: QubitRefSide, value: i16) {
        let mut conn = establish_connection();
        match side {
            QubitRefSide::Source => {
                self.src_measured = Some(value);
                change_src_measurement(&mut conn, self.id, value);
            }
            QubitRefSide::Destination => {
                self.dst_measured = Some(value);
                change_dst_measurement(&mut conn, self.id, value);
            }
        }
    }
}
