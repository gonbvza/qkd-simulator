use diesel::prelude::*;

use crate::{error::LinkError, models::links::Link, schema};

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
