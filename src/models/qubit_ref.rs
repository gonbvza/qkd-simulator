#[derive(Debug, Clone, Copy)]
pub enum QubitRefSide {
    Source,
    Destination,
}

#[derive(Debug, Clone)]
pub struct QubitRef {
    pub entangled_pair_id: i32,
    pub side: QubitRefSide,
}

impl QubitRef {
    pub fn new(entangled_pair_id: i32, side: QubitRefSide) -> QubitRef {
        QubitRef {
            entangled_pair_id,
            side,
        }
    }
}
