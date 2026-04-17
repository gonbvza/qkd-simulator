#[cfg(test)]
mod tests {
    use crate::models::detector::Detector;

    use super::*;

    fn make_detector(cooldown_ps: i64, last_detection_time: i64) -> Detector {
        Detector {
            id: 1,
            resolution_ps: 100,
            cooldown_ps,
            dark_count_rate: 0,
            last_detection_time,
        }
    }

    #[test]
    fn is_cooling_returns_true_within_cooldown_window() {
        let detector = make_detector(1000, 500);
        assert!(detector.is_cooling(1000));
    }

    #[test]
    fn is_cooling_returns_false_after_cooldown_expires() {
        let detector = make_detector(1000, 500);
        // current_time=1500 is not < 1500 → done cooling
        assert!(!detector.is_cooling(1500));
    }

    #[test]
    fn is_cooling_returns_false_well_past_cooldown() {
        let detector = make_detector(1000, 500);
        assert!(!detector.is_cooling(9999));
    }

    #[test]
    fn is_cooling_returns_true_immediately_after_detection() {
        let detector = make_detector(1000, 1000);
        // Fired at t=1000, queried at t=1001 — should still be cooling
        assert!(detector.is_cooling(1001));
    }

    #[test]
    fn is_cooling_false_when_never_used() {
        // last_detection_time=0, cooldown=1000 → cooling until t=1000
        let detector = make_detector(1000, 0);
        assert!(!detector.is_cooling(1000));
        assert!(detector.is_cooling(999));
    }
}
