use diesel::{insert_into, prelude::*};

use crate::{establish_connection, schema};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::node)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClientNode {
    pub id: i32,
    pub name: String,
    pub in_use: bool,
    pub measurements: i64,
    pub node_type: String,
}

impl ClientNode {
    pub fn new(_name: String) -> ClientNode {
        // TODO: Remove this call, create db pool
        let mut conn = establish_connection();

        let new_node: ClientNode = insert_into(schema::node::table)
            .values(schema::node::name.eq(_name))
            .get_result(&mut conn)
            .unwrap();

        println!("Created node {}", new_node.name);
        return new_node;
    }
}
