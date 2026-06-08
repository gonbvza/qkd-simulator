use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLIENT_NODE: String = "0".to_string();
    pub static ref EPR_NODE: String = "1".to_string();
}

/// Timeout used for pair events retry
pub const TIMEOUT: i64 = 1000;
/// Desired key length after qkd process
pub const KEY_LENGTH: usize = 1024;
/// Amounts of qubit sent by the epr node
pub const QUBIT_AMOUNT: i32 = 10000;
/// Default resolution for the detector
pub const DETECTOR_RESOLUTION: i64 = 10;
/// Default cooldown for the detector
pub const DETECTOR_COOLDOWN_US: i64 = 10;
/// Default cooldown rate for the detector
pub const DARK_COUNT_RATE: f64 = 100000.0;
/// Speed of light in fiber
pub const LIGHT_SPEED_FIBER: f64 = 2e8;
/// Constant for fidelity degradation in fiber
pub const FIDELITY_DEGRADATION: f32 = 14.0 / 10_000_000.0;
/// Time conversions
pub const SECONDS_TO_MICROSECONDS: i64 = 1_000_000;
pub const SECONDS_TO_MILISECONDS: i64 = 1_000;
/// Constant used for CHSH validation
pub const CHSH_THRESHOLD: f32 = 2_f32;
/// Constants used for basis mathematics
pub const FULL_CIRCLE: f64 = 360.0;
pub const HALF_CIRCLE: f64 = 180.0;
