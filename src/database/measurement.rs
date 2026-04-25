use diesel::{insert_into, prelude::*, PgConnection, RunQueryDsl};

use crate::{
    error::MeasurementError,
    models::{basis::Basis, measurement::Measurement},
    schema::{self},
};

pub fn create_new_measurement(
    conn: &mut PgConnection,
    node_id: i32,
    entangled_pair_id: i32,
    basis: Basis,
    value: i16,
    process_id: i32,
) -> Result<Measurement, MeasurementError> {
    let measurement: Measurement = insert_into(schema::measurements::table)
        .values((
            schema::measurements::node_id.eq(node_id),
            schema::measurements::entangled_pair_id.eq(entangled_pair_id),
            schema::measurements::basis.eq(basis),
            schema::measurements::value.eq(value),
            schema::measurements::process_id.eq(process_id),
        ))
        .returning(Measurement::as_returning())
        .get_result(conn)?;
    Ok(measurement)
}

pub fn get_measurement_for_pair(
    conn: &mut PgConnection,
    entangled_pair_id: i32,
) -> Result<Measurement, MeasurementError> {
    let measurement: Measurement = schema::measurements::table
        .filter(schema::measurements::entangled_pair_id.eq(entangled_pair_id))
        .select(Measurement::as_select())
        .get_result(conn)?;
    Ok(measurement)
}

pub fn set_accepted(conn: &mut PgConnection, id: i32) -> Result<(), MeasurementError> {
    diesel::update(schema::measurements::table)
        .filter(schema::measurements::id.eq(id))
        .set(schema::measurements::accepted.eq(true))
        .execute(conn)?;
    Ok(())
}

pub fn get_accepted_measurements(
    conn: &mut PgConnection,
    process_id: i32,
    node_id: i32,
) -> Result<Vec<Measurement>, MeasurementError> {
    let measurements: Vec<Measurement> = schema::measurements::table
        .filter(
            schema::measurements::node_id
                .eq(node_id)
                .and(schema::measurements::accepted.eq(true))
                .and(schema::measurements::process_id.eq(process_id)),
        )
        .select(Measurement::as_select())
        .load::<Measurement>(conn)?;

    Ok(measurements)
}
