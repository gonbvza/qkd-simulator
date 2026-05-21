use crate::{database::nodes::create_node, error::NodeError, nodes::node::Node};
use diesel::prelude::*;

impl Node {
    pub fn new(
        conn: &mut PgConnection,
        name: String,
        node_type: String,
        detector_id: i32,
    ) -> Result<Node, NodeError> {
        create_node(conn, &name, &node_type, detector_id)
    }

    pub fn get_id(&self) -> i32 {
        return self.id;
    }

    pub fn is_locked(&self) -> Option<i32> {
        self.locked_by
    }

    pub fn is_available_for(&self, process_id: i32) -> bool {
        self.locked_by.is_none() || self.locked_by == Some(process_id)
    }

    pub fn try_acquire(&mut self, process_id: i32) -> bool {
        match self.locked_by {
            None => {
                self.locked_by = Some(process_id);
                true
            }
            Some(owner) if owner == process_id => true,
            _ => false,
        }
    }

    pub fn release(&mut self, process_id: i32) -> Result<(), NodeError> {
        if self.locked_by == Some(process_id) {
            self.locked_by = None;
        }
        Ok(())
    }
}
