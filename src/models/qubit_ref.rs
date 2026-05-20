#[derive(Debug, Clone, Copy)]
pub enum QubitRefSide {
    Source,
    Destination,
}

#[derive(Debug, Clone)]
pub struct QubitRef {
    pub process_id: i32,
    pub qubit_nr: i32,
    pub side: QubitRefSide,
}

impl QubitRef {
    pub fn new(process_id: i32, qubit_nr: i32, side: QubitRefSide) -> QubitRef {
        QubitRef {
            process_id,
            qubit_nr,
            side,
        }
    }
}
