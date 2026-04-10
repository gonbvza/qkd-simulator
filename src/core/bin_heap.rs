use crate::models::event::Event;

pub struct BinHeap {
    pub arr: Vec<Event>,
}

impl BinHeap {
    pub fn new() -> BinHeap {
        BinHeap { arr: Vec::new() }
    }

    fn left(i: usize) -> usize {
        2 * i + 1
    }

    fn right(i: usize) -> usize {
        2 * i + 2
    }

    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    // Peek min
    pub fn get_min(&self) -> Option<Event> {
        self.arr.first().cloned()
    }

    pub fn insert(&mut self, event: Event) {
        // Push at end
        self.arr.push(event);

        // Bubble up
        let mut i = self.arr.len() - 1;

        while i > 0 {
            let p = Self::parent(i);

            if self.arr[i].timestamp < self.arr[p].timestamp {
                self.arr.swap(i, p);
                i = p;
            } else {
                break;
            }
        }
    }

    // Decrease the key (time) at index i
    pub fn decrease_key(&mut self, mut i: usize, new_event: Event) {
        self.arr[i] = new_event;

        // Bubble up
        while i > 0 {
            let p = Self::parent(i);

            if self.arr[i].timestamp < self.arr[p].timestamp {
                self.arr.swap(i, p);
                i = p;
            } else {
                break;
            }
        }
    }

    // Remove and return minimum element (root)
    pub fn extract_min(&mut self) -> Option<Event> {
        if self.arr.is_empty() {
            return None;
        }

        if self.arr.len() == 1 {
            return self.arr.pop();
        }

        let root = self.arr[0].clone();

        let last = self.arr.pop().unwrap();
        self.arr[0] = last;

        self.min_heapify(0);

        Some(root)
    }

    // Delete element at index i
    pub fn delete_key(&mut self, i: usize) {
        // Create a "minus infinity" event
        let mut min_event = self.arr[i].clone();
        min_event.timestamp = i32::MIN;

        self.decrease_key(i, min_event);
        self.extract_min();
    }

    // Heapify downward from index i
    fn min_heapify(&mut self, i: usize) {
        let n = self.arr.len();
        let l = Self::left(i);
        let r = Self::right(i);

        let mut smallest = i;

        if l < n && self.arr[l].timestamp < self.arr[smallest].timestamp {
            smallest = l;
        }

        if r < n && self.arr[r].timestamp < self.arr[smallest].timestamp {
            smallest = r;
        }

        if smallest != i {
            self.arr.swap(i, smallest);
            self.min_heapify(smallest);
        }
    }
}
