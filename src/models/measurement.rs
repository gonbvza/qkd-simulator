use diesel::prelude::*;

use crate::models::basis::Basis;

#[derive(Queryable, Selectable, Debug, Clone, Copy, PartialEq)]
#[diesel(table_name = crate::schema::measurements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
