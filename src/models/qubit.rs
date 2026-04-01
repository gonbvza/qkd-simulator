#[derive(Debug, Clone)]
pub enum Side {
    Right,
    Left,
}

#[derive(Debug, Clone)]
pub struct QubitRef {
    pub process_hash: String,
    pub entangled_pair_id: usize,
    pub side: Side,
}
