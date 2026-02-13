use crate::models::qubit::QubitRef;

#[derive(Debug, Clone)]
pub enum EventArgs {
    ArgStr(String),
    ArgInt(u32),
    QubitRef(QubitRef),
}
