use crate::{
    models::{
        links::Link,
        qubit_ref::{QubitRef, QubitRefSide},
    },
    nodes::node::{Node, NodeKind},
};

#[derive(Debug, Clone)]
pub enum EventArgs {
    ArgStr(String),
    ArgInt(u32),
    QubitRef(QubitRef),
    Node(Node),
    Link(Link),
    Number(i32),
    String(String),
    NodeType(NodeKind),
    Side(QubitRefSide),
}
