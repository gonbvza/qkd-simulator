use crate::test_func;

// Return all functions as (name, function pointer) tuples
pub fn get_event_functions() -> Vec<(&'static str, fn())> {
    vec![("test_func", test_func())]
}
