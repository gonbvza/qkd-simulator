use diesel::{insert_into, prelude::*, PgConnection, RunQueryDsl};

use crate::{error::PairError, models::entangled_pair::EntangledPair, schema};

pub fn create_entangled_pair(
    conn: &mut PgConnection,
    src_id: i32,
    dst_id: i32,
    current_time: i64,
    timeout_time: i64,
    process_id: i32,
    qubit_nr: i32,
) -> Result<EntangledPair, PairError> {
    let pair = insert_into(schema::entangled_pair::table)
        .values((
            schema::entangled_pair::src_id.eq(src_id),
            schema::entangled_pair::dst_id.eq(dst_id),
            schema::entangled_pair::created_at.eq(current_time),
            schema::entangled_pair::timeout_timestamp.eq(timeout_time),
            schema::entangled_pair::process_id.eq(process_id),
            schema::entangled_pair::qubit_nr.eq(qubit_nr),
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

pub fn change_src_measurement(
    conn: &mut PgConnection,
    id: i32,
    value: i16,
) -> Result<(), PairError> {
    diesel::update(schema::entangled_pair::table)
        .filter(schema::entangled_pair::id.eq(id))
        .set(schema::entangled_pair::src_measured.eq(value))
        .execute(conn)?;
    Ok(())
}

pub fn change_dst_measurement(
    conn: &mut PgConnection,
    id: i32,
    value: i16,
) -> Result<(), PairError> {
    diesel::update(schema::entangled_pair::table)
        .filter(schema::entangled_pair::id.eq(id))
        .set(schema::entangled_pair::dst_measured.eq(value))
        .execute(conn)?;
    Ok(())
}
