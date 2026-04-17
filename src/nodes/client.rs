use crate::{
    database::nodes::{create_node, lock_node},
    error::NodeError,
    establish_connection,
    models::{detector::Detector, measurement::Measurement},
    nodes::node::Node,
    schema::{self},
};
use diesel::prelude::*;

impl Node {
    pub fn new(
        conn: &mut PgConnection,
        name: String,
        node_type: String,
    ) -> Result<Node, NodeError> {
        let detector = Detector::new()?;
        create_node(conn, &name, &node_type, detector.id)
    }

    // TODO: MOVE THIS TO DB FILE
    pub fn get_measurements(&self) -> Result<Vec<Measurement>, NodeError> {
        let mut conn = establish_connection();

        let measurements: Vec<Measurement> = schema::measurements::table
            .select(Measurement::as_select())
            .filter(schema::measurements::node_id.eq(self.id))
            .load(&mut conn)?;

        Ok(measurements)
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
        let mut conn = establish_connection();
        match self.locked_by {
            None => {
                self.locked_by = Some(process_id);
                lock_node(&self, &mut conn, process_id);
                true
            }
            Some(owner) if owner == process_id => true,
            _ => false,
        }
    }

    pub fn release(&mut self, process_id: i32) {
        if self.locked_by == Some(process_id) {
            self.locked_by = None;
        }
    }
}
