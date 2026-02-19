use diesel::{dsl, insert_into, prelude::*, select};

use crate::{establish_connection, schema};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Link {
    pub id: i32,
    pub length: i64,
    pub attenuation: f32,
    pub error: f32,
    pub nodea: i32,
    pub nodeb: i32,
    pub next_available_time: i64,
}

impl Link {
    pub fn new(
        _length: i64,
        _attenuation: f32,
        _error: f32,
        _nodea: i32,
        _nodeb: i32,
    ) -> Option<Link> {
        // TODO: Remove this call, create db pool
        let mut conn = establish_connection();

        // Verify nodes exist
        let nodea_exists = select(dsl::exists(
            schema::nodes::table.filter(schema::nodes::id.eq(_nodea)),
        ))
        .get_result::<bool>(&mut conn)
        .unwrap();
        let nodeb_exists = select(dsl::exists(
            schema::nodes::table.filter(schema::nodes::id.eq(_nodeb)),
        ))
        .get_result::<bool>(&mut conn)
        .unwrap();

        if !nodea_exists || !nodeb_exists {
            // One of the nodes does not exist
            println!("Sorry one of the nodes does not exist");
            return None;
        }

        let link: Link = insert_into(schema::links::table)
            .values((
                schema::links::length.eq(_length),
                schema::links::attenuation.eq(_attenuation),
                schema::links::error.eq(_error),
                schema::links::nodea.eq(_nodea),
                schema::links::nodeb.eq(_nodeb),
                schema::links::next_available_time.eq(0),
            ))
            .get_result(&mut conn)
            .unwrap();

        return Some(link);
    }
}
