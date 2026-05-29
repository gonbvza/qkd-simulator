use std::sync::mpsc::channel;

use crate::{
    core::{event_loop::EventLoopHandler, registry::Registry},
    error::Error,
    models::{
        event::{Event, ScheduledEvent},
        event_types::{EventName, EventPayload},
    },
};

#[test]
pub fn test_missing_function() {
    let (tx, _) = channel();
    let handler = EventLoopHandler::new(tx);

    let mut registry = Registry::new();
    let event_now = Event::new_now(EventName::TestEvent, EventPayload::TestEvent());

    let ret = registry.exec_event(ScheduledEvent::new(event_now.clone(), 0), &handler);

    assert_eq!(ret.unwrap_err(), Error::FunctionNotFound(event_now.name));
}
