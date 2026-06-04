use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::models::node::NodeKind;

#[derive(Serialize, new)]
pub struct NodeDto {
    pub id: i32,
    pub node_type: NodeKind,
}

#[derive(Deserialize)]
pub struct CreateNodeDto {
    pub name: String,
    pub node_type: String,
}

#[derive(Deserialize)]
pub struct CreateLinkDto {
    pub src_id: i32,
    pub dst_id: i32,
    pub distance: i64,
    pub is_secure: bool,
}

#[derive(Serialize, new)]
pub struct LinkDto {
    pub src_id: i32,
    pub dst_id: i32,
    pub distance: i64,
    pub is_secure: bool,
}

#[derive(Serialize, new)]
pub struct StateDto {
    pub nodes: Vec<NodeDto>,
    pub links: Vec<LinkDto>,
}
