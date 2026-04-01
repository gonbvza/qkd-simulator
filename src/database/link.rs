use diesel::prelude::*;

use crate::{error::LinkError, models::links::Link, schema};

pub fn get_all_links(conn: &mut PgConnection) -> Result<Vec<Link>, LinkError> {
    let links: Vec<Link> = schema::links::table.load(conn)?;
    return Ok(links);
}
