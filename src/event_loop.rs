use crate::{
    models::{args::EventArgs, bin_heap::BinHeap, event::Event},
    registry::get_event_functions,
};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub struct EventLoop {
    pub bin_heap: BinHeap,
    pub funcs: HashMap<String, Box<dyn Fn(&HashMap<String, EventArgs>) + Send + Sync>>,
}

// The global singleton instance
static INSTANCE: OnceLock<Mutex<EventLoop>> = OnceLock::new();

impl EventLoop {
    /// Private constructor — only called once internally
    fn new() -> EventLoop {
        EventLoop {
            bin_heap: BinHeap::new(),
            funcs: HashMap::new(),
        }
    }

    /// Returns a reference to the global singleton's Mutex
    pub fn instance() -> &'static Mutex<EventLoop> {
        INSTANCE.get_or_init(|| Mutex::new(EventLoop::new()))
    }

    pub fn push_func<F>(&mut self, name: String, func: F)
    where
        F: Fn(&HashMap<String, EventArgs>) + Send + Sync + 'static,
    {
        self.funcs.insert(name, Box::new(func));
    }

    pub fn instantiate_functions(&mut self) {
        let funcs: Vec<(
            &'static str,
            Box<dyn Fn(&HashMap<String, EventArgs>) + Send + Sync>,
        )> = get_event_functions();
        for key in funcs {
            self.push_func(key.0.to_string(), key.1);
        }
    }

    pub fn exec_event(&mut self, event: Event) {
        match self.funcs.get(&event.function) {
            Some(function) => function(&event.args),
            None => println!("Function does not exist"),
        }
    }

    pub fn push_event(&mut self, event: Event) {
        self.bin_heap.insert(event);
    }

    pub fn run_loop(&mut self) {
        loop {
            let curr_event: Option<Event> = self.bin_heap.extract_min();
            if let Some(event) = curr_event {
                self.exec_event(event);
                break;
            }
        }
    }
}
