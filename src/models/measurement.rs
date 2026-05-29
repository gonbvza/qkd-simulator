use diesel::prelude::*;

use crate::models::basis::Basis;

/// Instance representing a measurement made by a node on a
/// single qubit of the pair
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
    /// Create new measurement
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

    /// Set the measurement as accepted
    ///
    /// An accepted measurement means that both qubits
    /// were received and measured correctly
    pub fn set_accepted(&mut self) {
        self.accepted = true;
    }

    /// Function to map measurement values of 0 to -1 for CHSH
    pub fn map_value(&mut self) {
        if self.value == 0 {
            self.value = -1
        }
    }
}
