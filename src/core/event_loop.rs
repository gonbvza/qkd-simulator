use crate::core::bin_heap::BinHeap;
use crate::models::{args::EventArgs, event::Event};
use std::sync::{Arc, Mutex, OnceLock};
use std::{collections::HashMap, sync::Condvar};

pub struct EventLoop {
    pub bin_heap: BinHeap,
}

static INSTANCE: OnceLock<Arc<(Mutex<EventLoop>, Condvar)>> = OnceLock::new();

impl EventLoop {
    fn new() -> EventLoop {
        EventLoop {
            bin_heap: BinHeap::new(),
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

    pub fn new_and_push(
        name: String,
        function: String,
        args: HashMap<String, EventArgs>,
        timestamp: i32,
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
