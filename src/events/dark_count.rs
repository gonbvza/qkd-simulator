use crate::{
    core::{event_loop::EventLoopHandler, state::SimulationState},
    error::{DetectorError, Error},
    models::{detector::Detector, event_types::EventPayload},
};

pub fn dark_count(
    payload: EventPayload,
    current_time: i64,
    state: &mut SimulationState,
    _handle: &EventLoopHandler,
) -> Result<(), Error> {
    // Get detector id
    let EventPayload::DarkCount(args) = payload else {
        return Err(Error::WrongArgs());
    };

    let detector: &mut Detector = state
        .get_detector_mut(args.detector_id)
        .ok_or(DetectorError::NotFound(args.detector_id))?;

    // Set cooldown received
    detector.set_detection_time(current_time)?;

    Ok(())
}
