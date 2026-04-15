use crate::models::args::EventArgs;
use std::collections::HashMap;

// Represents a scheduled event in the simulation with a timestamp (ps),
// a name for logging, a target function, and its arguments.
#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub function: String, // String representation of the function to execute
    pub args: HashMap<String, EventArgs>,
    pub timestamp: i64, // Event timestamp in ps
}

impl Event {
    pub fn new(
        name: String,
        function: String,
        args: HashMap<String, EventArgs>,
        timestamp: i64,
    ) -> Event {
        Event {
            name,
            function,
            args,
            timestamp,
        }
    }
}
