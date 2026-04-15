use diesel::{insert_into, prelude::*, PgConnection, RunQueryDsl};

use crate::{error::PairError, models::entangled_pair::EntangledPair, schema};

pub fn create_entangled_pair(
    conn: &mut PgConnection,
    src_id: i32,
    dst_id: i32,
    current_time: i64,
    timeout_time: i64,
) -> Result<EntangledPair, PairError> {
    let pair = insert_into(schema::entangled_pair::table)
        .values((
            schema::entangled_pair::src_id.eq(src_id),
            schema::entangled_pair::dst_id.eq(dst_id),
            schema::entangled_pair::created_at.eq(current_time),
            schema::entangled_pair::timeout_timestamp.eq(timeout_time),
        ))
        .get_result(conn)?;
    Ok(pair)
}
