use crate::{
    models::{args::EventArgs, bin_heap::BinHeap},
    registry::get_event_functions,
};
use std::collections::HashMap;

// Manages the simulation’s event queue using a priority heap.
// Events are retrieved in timestamp order to advance the global simulation time.
pub struct EventLoop {
    bin_heap: BinHeap,
    funcs: HashMap<String, Box<dyn Fn(&HashMap<String, EventArgs>)>>,
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
        while true {
            let curr_event: Option<Event> = self.bin_heap.extract_min();
            if let Some(event) = curr_event {
                self.exec_event(event);
                break;
            } else {
                println!("There is no evet");
            }
        }
    }
}

// Represents a scheduled event in the simulation with a timestamp (ps),
// a name for logging, a target function, and its arguments.
#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub function: String, // String representation of the function to execute
    pub args: HashMap<String, EventArgs>,
    pub timestamp: i32, // Event timestamp in ps
}

impl Event {
    pub fn new(
        name: String,
        function: String,
        args: HashMap<String, EventArgs>,
        timestamp: i32,
    ) -> Event {
        Event {
            name,
            function,
            args,
            timestamp,
        }
    }
}
