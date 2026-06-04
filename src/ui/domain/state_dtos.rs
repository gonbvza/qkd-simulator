use derive_new::new;
use serde::Serialize;

#[derive(Serialize, new)]
pub struct NodeDto {
    pub id: i32,
    pub node_type: String,
}

#[derive(Serialize, new)]
pub struct LinkDto {
    pub from_id: i32,
    pub to_id: i32,
    pub is_secure: bool,
}

#[derive(Serialize, new)]
pub struct StateDto {
    pub nodes: Vec<NodeDto>,
    pub links: Vec<LinkDto>,
}
