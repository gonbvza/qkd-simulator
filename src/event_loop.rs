use std::collections::HashMap;

use crate::{
    models::{args::EventArgs, bin_heap::BinHeap, event::Event},
    registry::get_event_functions,
};

// Manages the simulation’s event queue using a priority heap.
// Events are retrieved in timestamp order to advance the global simulation time.
pub struct EventLoop {
    pub bin_heap: BinHeap,
    pub funcs: HashMap<String, Box<dyn Fn(&HashMap<String, EventArgs>)>>,
}

impl EventLoop {
    pub fn new() -> EventLoop {
        EventLoop {
            bin_heap: BinHeap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn push_func<F>(&mut self, name: String, func: F)
    where
        F: Fn(&HashMap<String, EventArgs>) + 'static,
    {
        self.funcs.insert(name, Box::new(func));
    }

    pub fn instantiate_functions(&mut self) {
        let funcs: Vec<(&'static str, Box<dyn Fn(&HashMap<String, EventArgs>)>)> =
            get_event_functions();

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

    // Push event to the binary heap event loop
    pub fn push_event(&mut self, event: Event) {
        self.bin_heap.insert(event);
    }

    // Thread in charge of running event loop
    pub fn run_loop(&mut self) {
        // Add termination condition
        loop {
            let curr_event: Option<Event> = self.bin_heap.extract_min();
            if let Some(event) = curr_event {
                self.exec_event(event);
                break;
            }
        }
    }
}
