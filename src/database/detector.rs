use crate::{
    core::settings::{DETECTOR_COOLDOWN_US, DETECTOR_RESOLUTION},
    error::DetectorError,
    models::detector::Detector,
    schema,
};
use diesel::{insert_into, prelude::*, PgConnection, RunQueryDsl};

pub fn create_new_detector(conn: &mut PgConnection) -> Result<Detector, DetectorError> {
    let detector = insert_into(schema::detector::table)
        .values((
            schema::detector::resolution_ps.eq(DETECTOR_RESOLUTION),
            schema::detector::cooldown_ps.eq(DETECTOR_COOLDOWN_US),
            schema::detector::last_detection_time.eq(0),
        ))
        .get_result(conn)?;
    Ok(detector)
}
