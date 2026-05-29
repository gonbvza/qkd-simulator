use crate::models::event_types::{EventName, EventPayload};
use derive_new::new;

/// Time struct used to differ between events that must be
/// scheduled or executed now
#[derive(Debug, Clone, Copy)]
pub enum EventTime {
    At(i64),
    Now,
}

/// Instance representing an event that has been scheduled and
/// has to be executed
#[derive(Debug, Clone, new)]
pub struct Event {
    pub name: EventName,
    pub payload: EventPayload,
    pub timestamp: EventTime, // Event timestamp in ps
}

/// Instance representing the stored event in the event loop
///
/// Wrapper is used to resolve the EventTime struct and store
/// events based on a timestamp
#[derive(Debug, Clone, new)]
pub struct ScheduledEvent {
    pub event: Event,
    pub timestamp: i64,
}

impl EventTime {
    /// Resolve between executing now or at a specific timestamp
    pub fn resolve(self, current_time: i64) -> i64 {
        match self {
            EventTime::At(t) => t,
            EventTime::Now => current_time,
        }
    }
}

impl Event {
    /// Create event to be executed now
    pub fn new_now(name: EventName, payload: EventPayload) -> Event {
        Event::new(name, payload, EventTime::Now)
    }

    /// Create event to be executed at an specific timestamp
    pub fn new_at(name: EventName, payload: EventPayload, timestamp: i64) -> Event {
        Event::new(name, payload, EventTime::At(timestamp))
    }
}
