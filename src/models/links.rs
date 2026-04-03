use crate::{error::LinkError, establish_connection, schema};
use diesel::{dsl, insert_into, prelude::*, select};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Link {
    pub id: i32,
    pub length: i64,
    pub attenuation: f32,
    pub error_rate: f32,
    pub node_a: i32,
    pub node_b: i32,
    pub next_available_time: i64,
}

impl Link {
    pub fn new(
        length: i64,
        attenuation: f32,
        error_rate: f32,
        node_a: i32,
        node_b: i32,
    ) -> Result<Link, LinkError> {
        let mut conn = establish_connection();

        let node_a_exists = select(dsl::exists(
            schema::nodes::table.filter(schema::nodes::id.eq(node_a)),
        ))
        .get_result::<bool>(&mut conn)?;

        let node_b_exists = select(dsl::exists(
            schema::nodes::table.filter(schema::nodes::id.eq(node_b)),
        ))
        .get_result::<bool>(&mut conn)?;

        if !node_a_exists || !node_b_exists {
            return Err(LinkError::NonExistingNodes(node_a, node_b));
        }

        let link = insert_into(schema::links::table)
            .values((
                schema::links::length.eq(length),
                schema::links::attenuation.eq(attenuation),
                schema::links::error_rate.eq(error_rate),
                schema::links::node_a.eq(node_a),
                schema::links::node_b.eq(node_b),
                schema::links::next_available_time.eq(0),
            ))
            .get_result(&mut conn)?;

        Ok(link)
    }
}
