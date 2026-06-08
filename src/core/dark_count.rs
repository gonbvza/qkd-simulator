use crate::{
    core::{event_loop::EventLoopHandler, maths::calculate_dark_count_time},
    error::Error,
    models::{
        detector::Detector,
        event::Event,
        event_types::{DarkCountPayload, EventName, EventPayload},
    },
};

/// Function to schedule dark count events for a key generation process
///
/// All dark count events have to be schedule once at the begining for
/// performance concerns
pub fn schedule_dark_counts(
    detector: &Detector,
    current_time: i64,
    handle: &EventLoopHandler,
) -> Result<(), Error> {
    let mut time_accumulator = current_time;

    for ts in calculate_dark_count_time() {
        time_accumulator += ts;

        let event = Event::new_at(
            EventName::DarkCount,
            EventPayload::DarkCount(DarkCountPayload::new(detector.id)),
            time_accumulator,
        );

        handle.push_event(event)?;
    }

    Ok(())
}
