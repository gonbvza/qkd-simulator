use crate::{models::qubit::QubitRef, nodes::nodes::Node};

#[derive(Debug, Clone)]
pub enum EventArgs {
    ArgStr(String),
    ArgInt(u32),
    QubitRef(QubitRef),
    Node(Node),
}
