use diesel::prelude::*;

use crate::{error::MeasurementError, establish_connection, models::basis::Basis};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::measurements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measurement {
    pub node_id: i32,
    pub basis: Basis,
    pub process_id: i32,
    pub qubit_nr: i32,
    pub value: i16,
    pub accepted: bool,
}

impl Measurement {
    pub fn new(
        node_id: i32,
        qubit_nr: i32,
        basis: Basis,
        value: i16,
        process_id: i32,
    ) -> Measurement {
        Measurement {
            basis,
            node_id,
            qubit_nr,
            value,
            process_id,
            accepted: false,
        }
    }

    pub fn set_accepted(&mut self) {
        self.accepted = true;
    }
}
