use std::collections::HashMap;

use crate::{error::Error, events::qkd::handle_qkd_init, models::args::EventArgs};

// Return all functions as (name, function pointer) tuples

pub fn get_event_functions() -> Vec<(
    &'static str,
    Box<dyn Fn(&HashMap<String, EventArgs>) -> Result<(), Error> + Send + Sync>,
)> {
    vec![("handle_qkd_init", Box::new(handle_qkd_init))]
}
