use std::collections::HashMap;

use crate::models::event::{Event, EventLoop};

mod models;
mod registry;

fn test_func() {
    println!("Function is executed")
}

fn main() {
    // 1. Start priority heap event loop and store function instances
    let mut event_loop = EventLoop::new();
    event_loop.push_func(String::from("test_func"), test_func);
    let test_hm: HashMap<String, String> = HashMap::new();
    let test_event = Event::new(String::from("test"), String::from("test_func"), test_hm, 0);
    event_loop.exec_event(test_event);
}
