use crate::{core::process::Process, error::ProcessError, schema};
use diesel::{insert_into, prelude::*, PgConnection, RunQueryDsl};

pub fn create_new_process(
    conn: &mut PgConnection,
    current_time: i64,
) -> Result<Process, ProcessError> {
    let process = insert_into(schema::process::table)
        .values((schema::process::started_at.eq(current_time),))
        .get_result(conn)?;
    Ok(process)
}
