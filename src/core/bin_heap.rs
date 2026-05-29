use crate::models::event::ScheduledEvent;

/// Implementation of a binary heap for event queueing
pub struct BinHeap {
    pub arr: Vec<ScheduledEvent>,
}

impl BinHeap {
    pub fn new() -> BinHeap {
        BinHeap { arr: Vec::new() }
    }

    /// Get left child
    fn left(i: usize) -> usize {
        2 * i + 1
    }

    /// Get right child
    fn right(i: usize) -> usize {
        2 * i + 2
    }

    /// Get the parent of the node
    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    /// Insert item on the heap
    pub fn insert(&mut self, event: ScheduledEvent) {
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

    /// Remove and return minimum element (root)
    pub fn extract_min(&mut self) -> Option<ScheduledEvent> {
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

    /// Returns a reference to the event with the lowest timestamp
    pub fn get_min(&self) -> Option<&ScheduledEvent> {
        self.arr.first()
    }

    /// Update the event at `index` with `new_event` and restore heap order by bubbling up.
    pub fn decrease_key(&mut self, index: usize, new_event: ScheduledEvent) {
        if index >= self.arr.len() {
            return;
        }
        self.arr[index] = new_event;

        // Bubble up
        let mut i = index;
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

    /// Delete node from the heap
    pub fn delete_key(&mut self, index: usize) {
        if index >= self.arr.len() {
            return;
        }
        let last = self.arr.pop().unwrap();
        if index < self.arr.len() {
            self.arr[index] = last;
            let mut i = index;
            while i > 0 {
                let p = Self::parent(i);
                if self.arr[i].timestamp < self.arr[p].timestamp {
                    self.arr.swap(i, p);
                    i = p;
                } else {
                    break;
                }
            }
            if i == index {
                self.min_heapify(index);
            }
        }
    }

    /// Heapify downward from index i
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
