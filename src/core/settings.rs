use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLIENT_NODE: String = "0".to_string();
    pub static ref EPR_NODE: String = "1".to_string();
    pub static ref TIMEOUT: i64 = 1000;
    pub static ref KEY_LENGTH: i32 = 1024;
    pub static ref DETECTOR_RESOLUTION: i64 = 100;
    pub static ref DETECTOR_COOLDOWN_US: i64 = 300;
    pub static ref DETECTOR_DARK_COUNT_RATE: i32 = 10;
    pub static ref LIGHT_SPEED_FIBER: f64 = 2e8;
}
