use std::collections::HashMap;

// Manages the simulation’s event queue using a priority heap.
// Events are retrieved in timestamp order to advance the global simulation time.
#[derive(Debug)]
pub struct EventLoop {
    bin_heap: Vec<Event>,
    funcs: HashMap<String, fn()>,
}

impl EventLoop {
    pub fn new() -> EventLoop {
        EventLoop {
            bin_heap: Vec::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn push_func(&mut self, name: String, func: fn()) {
        self.funcs.insert(name, func);
    }

    pub fn exec_event(&mut self, event: Event) {
        match self.funcs.get(event.function.as_str()) {
            Some(function) => function(),
            None => println!("Function does not exist"),
        }
    }
}

// Represents a scheduled event in the simulation with a timestamp (ps),
// a name for logging, a target function, and its arguments.
#[derive(Debug)]
pub struct Event {
    name: String,
    function: String, // String representation of the function to execute
    args: HashMap<String, String>,
    timestamp: i32, // Event timestamp in ps
}

impl Event {
    pub fn new(
        name: String,
        function: String,
        args: HashMap<String, String>,
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
