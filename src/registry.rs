use std::collections::HashMap;

use crate::{models::args::EventArgs, test_func};

// Return all functions as (name, function pointer) tuples

pub fn get_event_functions() -> Vec<(
    &'static str,
    Box<dyn Fn(&HashMap<String, EventArgs>) + Send + Sync>,
)> {
    vec![("test_func", Box::new(test_func))]
}
