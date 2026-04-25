use diesel::prelude::*;

use crate::{
    database::measurement::{create_new_measurement, set_accepted},
    error::MeasurementError,
    establish_connection,
    models::basis::Basis,
};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::measurements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measurement {
    pub id: i32,
    pub node_id: i32,
    pub basis: Basis,
    pub entangled_pair_id: i32,
    pub value: i16,
    pub accepted: bool,
}

impl Measurement {
    pub fn new(
        node_id: i32,
        entangled_pair_id: i32,
        basis: Basis,
        value: i16,
        process_id: i32,
    ) -> Result<Measurement, MeasurementError> {
        let mut conn = establish_connection();
        create_new_measurement(
            &mut conn,
            node_id,
            entangled_pair_id,
            basis,
            value,
            process_id,
        )
    }

    pub fn set_accepted(&mut self) -> Result<(), MeasurementError> {
        let mut conn = establish_connection();
        self.accepted = true;
        set_accepted(&mut conn, self.id)?;
        Ok(())
    }
}
