use crate::models::event_types::{EventName, EventPayload};
use derive_new::new;

#[derive(Debug, Clone, Copy)]
pub enum EventTime {
    At(i64),
    Now,
}

impl EventTime {
    pub fn resolve(self, current_time: i64) -> i64 {
        match self {
            EventTime::At(t) => t,
            EventTime::Now => current_time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub name: EventName,
    pub payload: EventPayload,
    pub timestamp: EventTime, // Event timestamp in ps
}

#[derive(Debug, Clone, new)]
pub struct ScheduledEvent {
    pub event: Event,
    pub timestamp: i64,
}

impl Event {
    pub fn new(name: EventName, payload: EventPayload, timestamp: EventTime) -> Event {
        Event {
            name,
            payload,
            timestamp,
        }
    }

    pub fn new_now(name: EventName, payload: EventPayload) -> Event {
        Event::new(name, payload, EventTime::Now)
    }

    pub fn new_at(name: EventName, payload: EventPayload, timestamp: i64) -> Event {
        Event::new(name, payload, EventTime::At(timestamp))
    }
}
