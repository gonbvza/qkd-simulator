use diesel::{insert_into, prelude::*, result::Error};

use crate::{
    error::{map_db_error, NodeError},
    establish_connection,
    schema::{self},
};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClientNode {
    pub id: i32,
    pub name: String,
    pub in_use: bool,
    pub measurements: i64,
    pub node_type: String,
}

impl ClientNode {
    pub fn new(_name: String) -> Result<ClientNode, NodeError> {
        // TODO: Remove this call, create db pool
        let mut conn = establish_connection();

        let new_node: Result<ClientNode, Error> = insert_into(schema::nodes::table)
            .values(schema::nodes::name.eq(_name.clone()))
            .get_result(&mut conn);

        match new_node {
            Ok(node) => Ok(node),
            Err(e) => Err(map_db_error(_name.clone(), e)),
        }
    }
}
