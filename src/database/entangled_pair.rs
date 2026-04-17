use diesel::{insert_into, prelude::*, PgConnection, RunQueryDsl};

use crate::{error::PairError, models::entangled_pair::EntangledPair, schema};

pub fn create_entangled_pair(
    conn: &mut PgConnection,
    src_id: i32,
    dst_id: i32,
    current_time: i64,
    timeout_time: i64,
    process_id: i32,
) -> Result<EntangledPair, PairError> {
    let pair = insert_into(schema::entangled_pair::table)
        .values((
            schema::entangled_pair::src_id.eq(src_id),
            schema::entangled_pair::dst_id.eq(dst_id),
            schema::entangled_pair::created_at.eq(current_time),
            schema::entangled_pair::timeout_timestamp.eq(timeout_time),
            schema::entangled_pair::process_id.eq(process_id),
        ))
        .get_result(conn)?;
    Ok(pair)
}

pub fn get_pair_by_id(
    conn: &mut PgConnection,
    entangled_pair_id: i32,
) -> Result<EntangledPair, PairError> {
    let entangled_pair: EntangledPair = schema::entangled_pair::table
        .filter(schema::entangled_pair::id.eq(entangled_pair_id))
        .first(conn)?;
    Ok(entangled_pair)
}
