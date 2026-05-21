use std::error;

use diesel::{insert_into, prelude::*};

use crate::{
    error::LinkError,
    models::links::Link,
    schema::{self},
};

pub fn create_link(
    conn: &mut PgConnection,
    length: i64,
    attenuation: f32,
    error_rate: f32,
    src_id: i32,
    dst_id: i32,
) -> Result<Link, LinkError> {
    let link = insert_into(schema::links::table)
        .values((
            schema::links::length.eq(length),
            schema::links::attenuation.eq(attenuation),
            schema::links::error_rate.eq(error_rate),
            schema::links::src_id.eq(src_id),
            schema::links::dst_id.eq(dst_id),
            schema::links::next_available_time.eq(0),
        ))
        .get_result(conn)?;
    Ok(link)
}

pub fn get_all_links(conn: &mut PgConnection) -> Result<Vec<Link>, LinkError> {
    let links: Vec<Link> = schema::links::table.load(conn)?;
    return Ok(links);
}

pub fn get_link(conn: &mut PgConnection, src_id: i32, dst_id: i32) -> Result<Link, LinkError> {
    let link: Link = schema::links::table
        .filter(
            schema::links::src_id
                .eq(src_id)
                .and(schema::links::dst_id.eq(dst_id))
                .or(schema::links::src_id
                    .eq(dst_id)
                    .and(schema::links::dst_id.eq(src_id))),
        )
        .first(conn)?;
    Ok(link)
}
