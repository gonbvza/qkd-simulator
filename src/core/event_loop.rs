use crate::core::bin_heap::BinHeap;
use crate::core::registry::Registry;
use crate::error::Error;
use crate::error::SimError;
use crate::models::event::{Event, ScheduledEvent};
use std::sync::mpsc::{Receiver, Sender};

/// Handle used by producers to enqueue events for the simulation loop.
#[derive(Debug, Clone)]
pub struct EventLoopHandler {
    tx: Sender<Event>,
}

impl EventLoopHandler {
    /// Creates a new event-loop handle backed by the given sender.
    pub fn new(tx: Sender<Event>) -> EventLoopHandler {
        EventLoopHandler { tx }
    }

    /// Sends an event to the loop.
    pub fn push_event(&self, event: Event) -> Result<(), SimError> {
        self.tx.send(event)?;
        Ok(())
    }
}

/// Priority queue and simulated time for pending events.
pub struct EventLoop {
    bin_heap: BinHeap,
    current_time: i64,
}

impl EventLoop {
    /// Creates an empty event loop starting at time zero.
    pub fn new() -> EventLoop {
        EventLoop {
            bin_heap: BinHeap::new(),
            current_time: 0,
        }
    }

    /// Schedules an event relative to the loop's current time.
    pub fn push_event(&mut self, event: Event) {
        let timestamp = event.timestamp.resolve(self.current_time);
        let schedule_event = ScheduledEvent::new(event, timestamp);
        self.bin_heap.insert(schedule_event);
    }

    /// Removes the next scheduled event, if any.
    pub fn pop_next_event(&mut self) -> Option<ScheduledEvent> {
        self.bin_heap.extract_min()
    }

    /// Updates the loop's current time and returns the new value.
    pub fn set_new_timestamp(&mut self, new_time: i64) -> i64 {
        self.current_time = new_time;
        self.current_time
    }

    /// Returns the current simulated time.
    pub fn get_current_time(&self) -> i64 {
        self.current_time
    }

    /// Runs the simulation loop until the event queue and channel are both exhausted.
    ///
    /// Processes all queued events in timestamp order. If the heap is empty,
    /// the loop blocks waiting for a new event on the channel. Returns `Ok(())`
    /// when the channel is closed and no events remain.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if event execution fails unrecoverably. Non-fatal
    /// event errors are logged to stderr and the loop continues.
    pub fn run_loop(
        &mut self,
        mut registry: Registry,
        handle: EventLoopHandler,
        rx: Receiver<Event>,
    ) -> Result<(), Error> {
        loop {
            // Loop through all queued events
            while let Ok(event) = rx.try_recv() {
                self.push_event(event);
            }

            let Some(scheduled_event) = self.pop_next_event() else {
                // No event in heap, wait until event is sent 
                match rx.recv() {
                    Ok(event) => {
                        self.push_event(event);
                        continue;
                    }
                    Err(_) => return Ok(()),
                }
            };

            let timestamp = scheduled_event.timestamp;
            self.set_new_timestamp(timestamp);
            let event_name = scheduled_event.event.name.clone();

            if let Err(e) = registry.exec_event(scheduled_event, &handle) {
                eprintln!(
                    "Event {:?} at t={} failed; continuing loop: {}",
                    event_name, timestamp, e
                );
            }
        }
    }
}
