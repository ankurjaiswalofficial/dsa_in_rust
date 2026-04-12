//! Double-ended queue backed by `VecDeque<T>`.
//!
//! A deque supports insertion and removal at both ends. It is useful for
//! sliding-window extrema, BFS variants, and scheduling tasks where both ends
//! matter.
//!
//! # Example
//!
//! ```
//! use project1::data_structures::deque::Deque;
//!
//! let mut deque = Deque::new();
//! deque.push_back(2);
//! deque.push_front(1);
//! deque.push_back(3);
//! assert_eq!(deque.pop_front(), Some(1));
//! assert_eq!(deque.pop_back(), Some(3));
//! ```

use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
pub struct Deque<T> {
    items: VecDeque<T>,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.items.push_front(value);
    }

    pub fn push_back(&mut self, value: T) {
        self.items.push_back(value);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.items.pop_back()
    }

    pub fn front(&self) -> Option<&T> {
        self.items.front()
    }

    pub fn back(&self) -> Option<&T> {
        self.items.back()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Deque;

    #[test]
    fn deque_supports_both_ends() {
        let mut deque = Deque::new();
        deque.push_back(2);
        deque.push_front(1);
        deque.push_back(3);
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));
        assert!(deque.is_empty());
    }
}
