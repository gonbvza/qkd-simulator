use crate::core::bin_heap::BinHeap;
use crate::models::{args::EventArgs, event::Event};
use std::sync::{Arc, Mutex, OnceLock};
use std::{collections::HashMap, sync::Condvar};

pub struct EventLoop {
    pub bin_heap: BinHeap,
    pub current_time: i64,
}

static INSTANCE: OnceLock<Arc<(Mutex<EventLoop>, Condvar)>> = OnceLock::new();

impl EventLoop {
    fn new() -> EventLoop {
        EventLoop {
            bin_heap: BinHeap::new(),
            current_time: 0,
        }
    }

    pub fn instance() -> &'static Arc<(Mutex<EventLoop>, Condvar)> {
        INSTANCE.get_or_init(|| Arc::new((Mutex::new(EventLoop::new()), Condvar::new())))
    }

    pub fn push_event(&mut self, event: Event) {
        self.bin_heap.insert(event);

        let pair = EventLoop::instance().clone();
        let (_, cvar) = &*pair;
        cvar.notify_one();
    }

    pub fn set_new_timestamp(&mut self, delta: &i64) -> i64 {
        self.current_time = self.current_time + delta;
        return self.current_time;
    }

    pub fn get_current_time(&mut self) -> i64 {
        return self.current_time;
    }

    pub fn new_and_push(
        name: String,
        function: String,
        args: HashMap<String, EventArgs>,
        timestamp: i64,
    ) {
        let event = Event::new(name, function, args, timestamp);
        let pair = Arc::clone(EventLoop::instance());
        let (event_loop, cvar) = &*pair;
        {
            event_loop
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .bin_heap
                .insert(event);
        }
        cvar.notify_one();
    }
}
