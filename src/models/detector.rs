use diesel::prelude::*;

use crate::{database::detector::create_new_detector, error::DetectorError, establish_connection};

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::detector)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Detector {
    pub id: i32,
    pub resolution_ps: i64,
    pub cooldown_ps: i64,
    pub dark_count_rate: i32,
    pub last_detection_time: i64,
}

impl Detector {
    pub fn new() -> Result<Detector, DetectorError> {
        let mut conn = establish_connection();
        create_new_detector(&mut conn)
    }

    pub fn is_cooling(&self, current_time: i64) -> bool {
        current_time < (self.last_detection_time + self.cooldown_ps)
    }

    pub fn set_detection_time(&mut self, current_time: i64) -> Result<(), DetectorError> {
        self.last_detection_time = current_time;
        Ok(())
    }
}
