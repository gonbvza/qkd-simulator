use crate::{
    database::nodes::create_node,
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

    pub fn set_in_use(&mut self, value: bool) {
        self.in_use = value;
    }
}
