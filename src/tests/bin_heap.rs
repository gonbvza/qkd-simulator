#[cfg(test)]
mod tests {
    use crate::models::{bin_heap::BinHeap, event::Event};

    use super::*;
    use std::collections::HashMap;

    fn make_event(ts: i32) -> Event {
        Event {
            name: format!("e{}", ts),
            function: "noop".to_string(),
            args: HashMap::new(),
            timestamp: ts,
        }
    }

    #[test]
    fn new_heap_is_empty() {
        let heap = BinHeap::new();
        assert!(heap.arr.is_empty());
    }

    #[test]
    fn insert_single_event() {
        let mut heap = BinHeap::new();

        heap.insert(make_event(10));

        assert_eq!(heap.arr.len(), 1);
        assert_eq!(heap.get_min().unwrap().timestamp, 10);
    }

    #[test]
    fn insert_orders_by_timestamp() {
        let mut heap = BinHeap::new();

        heap.insert(make_event(30));
        heap.insert(make_event(10));
        heap.insert(make_event(20));

        assert_eq!(heap.get_min().unwrap().timestamp, 10);
    }

    #[test]
    fn extract_min_returns_smallest() {
        let mut heap = BinHeap::new();

        heap.insert(make_event(5));
        heap.insert(make_event(1));
        heap.insert(make_event(3));

        let min = heap.extract_min().unwrap();
        assert_eq!(min.timestamp, 1);
    }

    #[test]
    fn extract_min_multiple_times_keeps_order() {
        let mut heap = BinHeap::new();

        heap.insert(make_event(4));
        heap.insert(make_event(2));
        heap.insert(make_event(1));
        heap.insert(make_event(3));

        let mut result = Vec::new();

        while let Some(e) = heap.extract_min() {
            result.push(e.timestamp);
        }

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn extract_min_empty_returns_none() {
        let mut heap = BinHeap::new();
        assert!(heap.extract_min().is_none());
    }

    #[test]
    fn decrease_key_moves_element_up() {
        let mut heap = BinHeap::new();

        heap.insert(make_event(20));
        heap.insert(make_event(30));
        heap.insert(make_event(40));

        // decrease index 2 to timestamp 5
        heap.decrease_key(2, make_event(5));

        assert_eq!(heap.get_min().unwrap().timestamp, 5);
    }

    #[test]
    fn delete_key_removes_element() {
        let mut heap = BinHeap::new();

        heap.insert(make_event(10));
        heap.insert(make_event(20));
        heap.insert(make_event(30));

        // delete middle element
        heap.delete_key(1);

        let mut vals = vec![];
        while let Some(e) = heap.extract_min() {
            vals.push(e.timestamp);
        }

        assert_eq!(vals, vec![10, 30]);
    }

    #[test]
    fn get_min_does_not_remove() {
        let mut heap = BinHeap::new();

        heap.insert(make_event(7));
        heap.insert(make_event(3));

        let min = heap.get_min().unwrap();
        assert_eq!(min.timestamp, 3);

        // Still there
        assert_eq!(heap.arr.len(), 2);
    }
}
