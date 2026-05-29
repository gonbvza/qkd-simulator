use crate::{
    core::event_loop::EventLoop,
    models::{
        event::{Event, ScheduledEvent},
        event_types::{EventName, EventPayload},
    },
};

#[test]
fn test_push_now() {
    let mut event_loop = EventLoop::new();
    let event = Event::new_now(EventName::TestEvent, EventPayload::TestEvent());
    event_loop.push_event(event);
    let pop_event: Option<ScheduledEvent> = event_loop.pop_next_event();
    assert!(pop_event.is_some());
    assert_eq!(pop_event.unwrap().timestamp, 0);
}

#[test]
fn test_push_event_at() {
    let mut event_loop = EventLoop::new();
    let event = Event::new_at(EventName::TestEvent, EventPayload::TestEvent(), 100);
    event_loop.push_event(event);
    let pop_event: Option<ScheduledEvent> = event_loop.pop_next_event();
    assert!(pop_event.is_some());
    assert_eq!(pop_event.unwrap().timestamp, 100);
}

#[test]
fn test_push_event_ordering() {
    let mut event_loop = EventLoop::new();
    let event_at = Event::new_at(EventName::TestEvent, EventPayload::TestEvent(), 100);
    event_loop.push_event(event_at);
    let event_now = Event::new_now(EventName::TestEvent, EventPayload::TestEvent());
    event_loop.push_event(event_now);

    let pop_event_now: Option<ScheduledEvent> = event_loop.pop_next_event();
    assert!(pop_event_now.is_some());
    assert_eq!(pop_event_now.unwrap().timestamp, 0);

    let pop_event_at: Option<ScheduledEvent> = event_loop.pop_next_event();
    assert!(pop_event_at.is_some());
    assert_eq!(pop_event_at.unwrap().timestamp, 100);
}
