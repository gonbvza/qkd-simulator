use crate::{
    core::settings::{DETECTOR_DARK_COUNT_RATE, DETECTOR_RESOLUTION},
    error::DetectorError,
    models::detector::Detector,
    schema,
};
use diesel::{insert_into, prelude::*, PgConnection, RunQueryDsl};

pub fn create_new_detector(conn: &mut PgConnection) -> Result<Detector, DetectorError> {
    let detector = insert_into(schema::detector::table)
        .values((
            schema::detector::resolution_ps.eq(DETECTOR_RESOLUTION.clone()),
            schema::detector::cooldown_ps.eq(0),
            schema::detector::dark_count_rate.eq(DETECTOR_DARK_COUNT_RATE.clone()),
            schema::detector::last_detection_time.eq(0),
        ))
        .get_result(conn)?;
    Ok(detector)
}

pub fn get_detector_by_id(
    conn: &mut PgConnection,
    detector_id: i32,
) -> Result<Detector, DetectorError> {
    let detector: Detector = schema::detector::table
        .filter(schema::detector::id.eq(detector_id))
        .get_result(conn)?;
    Ok(detector)
}
