use crate::models::args::EventArgs;
use std::{collections::HashMap, i64};

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
    pub name: String,
    pub function: String, // String representation of the function to execute
    pub args: HashMap<String, EventArgs>,
    pub timestamp: EventTime, // Event timestamp in ps
}

#[derive(Debug, Clone)]
pub struct ScheduledEvent {
    pub event: Event,
    pub timestamp: i64,
}

impl Event {
    pub fn new(
        name: String,
        function: String,
        args: HashMap<String, EventArgs>,
        timestamp: EventTime,
    ) -> Event {
        Event {
            name,
            function,
            args,
            timestamp,
        }
    }

    pub fn new_now(name: String, function: String, args: HashMap<String, EventArgs>) -> Event {
        Event::new(name, function, args, EventTime::Now)
    }

    pub fn new_at(
        name: String,
        function: String,
        args: HashMap<String, EventArgs>,
        timestamp: i64,
    ) -> Event {
        Event::new(name, function, args, EventTime::At(timestamp))
    }
}
