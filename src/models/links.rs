use std::fmt;

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
    pub src_id: i32,
    pub dst_id: i32,
    pub next_available_time: i64,
}

impl Link {
    pub fn new(
        length: i64,
        attenuation: f32,
        error_rate: f32,
        src_id: i32,
        dst_id: i32,
    ) -> Result<Link, LinkError> {
        let mut conn = establish_connection();

        let node_a_exists = select(dsl::exists(
            schema::nodes::table.filter(schema::nodes::id.eq(src_id)),
        ))
        .get_result::<bool>(&mut conn)?;

        let node_b_exists = select(dsl::exists(
            schema::nodes::table.filter(schema::nodes::id.eq(dst_id)),
        ))
        .get_result::<bool>(&mut conn)?;

        if !node_a_exists || !node_b_exists {
            return Err(LinkError::NonExistingNodes(src_id, dst_id));
        }

        let link = insert_into(schema::links::table)
            .values((
                schema::links::length.eq(length),
                schema::links::attenuation.eq(attenuation),
                schema::links::error_rate.eq(error_rate),
                schema::links::src_id.eq(src_id),
                schema::links::dst_id.eq(dst_id),
                schema::links::next_available_time.eq(0),
            ))
            .get_result(&mut conn)?;

        Ok(link)
    }

    pub fn get_link(node_a_id: i32, node_b_id: i32) -> Result<Link, LinkError> {
        let mut conn = establish_connection();
        let link: Link = schema::links::table
            .filter(
                schema::links::src_id
                    .eq(node_a_id)
                    .and(schema::links::dst_id.eq(node_b_id)),
            )
            .first(&mut conn)?;
        Ok(link)
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] Node {} ↔ Node {} | Length: {}m | Attenuation: {:.2} dB | Error Rate: {:.4}% | Next Available: {}",
            self.id,
            self.src_id,
            self.dst_id,
            self.length,
            self.attenuation,
            self.error_rate * 100.0,
            if self.next_available_time == 0 {
                "Now".to_string()
            } else {
                format!("t={}", self.next_available_time)
            }
        )
    }
}
