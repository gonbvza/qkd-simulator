use crate::{
    models::{links::Link, qubit_ref::QubitRef},
    nodes::node::Node,
};

#[derive(Debug, Clone)]
pub enum EventArgs {
    ArgStr(String),
    ArgInt(u32),
    QubitRef(QubitRef),
    Node(Node),
    Link(Link),
}
