use crate::{
    error::NodeError,
    establish_connection,
    models::measurement::Measurement,
    nodes::{
        common::create_client_node,
        nodes::{ClientNode, NodeUsage},
    },
    schema::{self},
};
use diesel::prelude::*;

impl ClientNode {
    pub fn new(conn: &mut PgConnection, name: String) -> Result<ClientNode, NodeError> {
        create_client_node(conn, &name)
    }

    pub fn get_measurements(&self) -> Result<Vec<Measurement>, NodeError> {
        let mut conn = establish_connection();

        let measurements: Vec<Measurement> = schema::measurements::table
            .select(Measurement::as_select())
            .filter(schema::measurements::node_id.eq(self.id))
            .load(&mut conn)?;

        Ok(measurements)
    }
}

impl NodeUsage for ClientNode {
    fn get_id(&self) -> i32 {
        return self.id;
    }

    fn set_in_use(&mut self, value: bool) {
        self.in_use = value;
    }
}
