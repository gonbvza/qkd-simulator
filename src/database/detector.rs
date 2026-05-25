use crate::{
    core::settings::{DETECTOR_COOLDOWN_US, DETECTOR_DARK_COUNT_RATE, DETECTOR_RESOLUTION},
    error::DetectorError,
    models::detector::Detector,
    schema,
};
use diesel::{PgConnection, RunQueryDsl, insert_into, prelude::*};

pub fn create_new_detector(conn: &mut PgConnection) -> Result<Detector, DetectorError> {
    let detector = insert_into(schema::detector::table)
        .values((
            schema::detector::resolution_ps.eq(DETECTOR_RESOLUTION),
            schema::detector::cooldown_ps.eq(DETECTOR_COOLDOWN_US),
            schema::detector::dark_count_rate.eq(DETECTOR_DARK_COUNT_RATE),
            schema::detector::last_detection_time.eq(0),
        ))
        .get_result(conn)?;
    Ok(detector)
}
