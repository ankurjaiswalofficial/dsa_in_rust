//! Queue implementation backed by `VecDeque<T>`.
//!
//! A queue is first-in, first-out (FIFO). It appears in scheduling, BFS, and buffering.
//!
//! # Example
//!
//! ```
//! use project1::data_structures::queue::{CircularQueue, Queue};
//!
//! let mut queue = Queue::new();
//! queue.enqueue("a");
//! queue.enqueue("b");
//! assert_eq!(queue.dequeue(), Some("a"));
//!
//! let mut circular = CircularQueue::new(3);
//! assert!(circular.enqueue(1));
//! ```

use crate::data_structures::heap::MinHeap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.items.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn front(&self) -> Option<&T> {
        self.items.front()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct CircularQueue<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    len: usize,
}

impl<T> CircularQueue<T> {
    pub fn new(capacity: usize) -> Self {
        let data = std::iter::repeat_with(|| None).take(capacity.max(1)).collect();
        Self {
            data,
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    pub fn enqueue(&mut self, value: T) -> bool {
        if self.len == self.data.len() {
            return false;
        }
        self.data[self.tail] = Some(value);
        self.tail = (self.tail + 1) % self.data.len();
        self.len += 1;
        true
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let value = self.data[self.head].take();
        self.head = (self.head + 1) % self.data.len();
        self.len -= 1;
        value
    }
}

#[derive(Debug, Clone, Default)]
pub struct PriorityQueue<T: Ord> {
    heap: MinHeap<T>,
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        Self {
            heap: MinHeap::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.heap.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::{CircularQueue, PriorityQueue, Queue};

    #[test]
    fn queue_is_fifo() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
    }

    #[test]
    fn circular_and_priority_queue_work() {
        let mut circular = CircularQueue::new(3);
        assert!(circular.enqueue(1));
        assert!(circular.enqueue(2));
        assert!(circular.enqueue(3));
        assert!(!circular.enqueue(4));
        assert_eq!(circular.dequeue(), Some(1));
        assert!(circular.enqueue(4));
        assert_eq!(circular.dequeue(), Some(2));

        let mut pq = PriorityQueue::new();
        pq.push(5);
        pq.push(1);
        pq.push(3);
        assert_eq!(pq.pop(), Some(1));
    }
}
