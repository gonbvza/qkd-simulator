use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLIENT_NODE: String = "0".to_string();
    pub static ref EPR_NODE: String = "1".to_string();
}

pub const TIMEOUT: i64 = 1000;
pub const KEY_LENGTH: i32 = 1024;
pub const QUBIT_AMOUNT: i32 = 10000;
pub const DETECTOR_RESOLUTION: i64 = 100;
pub const DETECTOR_COOLDOWN_US: i64 = 300;
pub const DETECTOR_DARK_COUNT_RATE: i32 = 10;
pub const LIGHT_SPEED_FIBER: f64 = 2e8;
pub const FIDELITY_DEGRADATION: f32 = 14.0 / 10_000_000.0;
pub const FULL_CIRCLE: f64 = 360.0;
pub const HALF_CIRCLE: f64 = 180.0;
