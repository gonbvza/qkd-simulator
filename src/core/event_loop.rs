use crate::core::bin_heap::BinHeap;
use crate::error::SimError;
use crate::models::event::{Event, ScheduledEvent};
use std::sync::mpsc::Sender;

// This struct will hold a channel to push events to the event loop thread
#[derive(Debug, Clone)]
pub struct EventLoopHandler {
    tx: Sender<Event>,
}

impl EventLoopHandler {
    pub fn new(tx: Sender<Event>) -> EventLoopHandler {
        EventLoopHandler { tx }
    }

    pub fn push_event(&self, event: Event) -> Result<(), SimError> {
        self.tx.send(event)?;
        Ok(())
    }
}

pub struct EventLoop {
    bin_heap: BinHeap,
    current_time: i64,
}

impl EventLoop {
    pub fn new() -> EventLoop {
        EventLoop {
            bin_heap: BinHeap::new(),
            current_time: 0,
        }
    }

    pub fn push_event(&mut self, event: Event) {
        let timestamp = event.timestamp.resolve(self.current_time);
        let schedule_event = ScheduledEvent::new(event, timestamp);
        self.bin_heap.insert(schedule_event);
    }

    pub fn pop_next_event(&mut self) -> Option<ScheduledEvent> {
        self.bin_heap.extract_min()
    }

    pub fn set_new_timestamp(&mut self, new_time: i64) -> i64 {
        self.current_time = new_time;
        self.current_time
    }

    pub fn get_current_time(&self) -> i64 {
        self.current_time
    }
}
