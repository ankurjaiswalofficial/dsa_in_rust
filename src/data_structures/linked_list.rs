//! Singly linked list.
//!
//! Linked lists make insertion at the head cheap but are less cache-friendly than arrays.
//!
//! # Example
//!
//! ```
//! use project1::data_structures::linked_list::SinglyLinkedList;
//!
//! let mut list = SinglyLinkedList::new();
//! list.push_front(2);
//! list.push_front(1);
//! assert_eq!(list.pop_front(), Some(1));
//! assert_eq!(list.to_vec(), vec![2]);
//! ```

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

#[derive(Debug, Clone, Default)]
pub struct SinglyLinkedList<T> {
    head: Link<T>,
    len: usize,
}

/// A simple educational doubly linked list model.
///
/// This uses a `Vec<T>` backing store instead of unsafe pointers so the repo can
/// focus on the semantics of double-ended operations without introducing unsafe
/// Rust in the basics phase.
#[derive(Debug, Clone, Default)]
pub struct DoublyLinkedList<T> {
    items: Vec<T>,
}

/// Circular linked lists conceptually connect the tail back to the head.
///
/// The implementation below stores the values and exposes a method that walks
/// them as if the sequence were circular.
#[derive(Debug, Clone, Default)]
pub struct CircularLinkedList<T> {
    items: Vec<T>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn push_front(&mut self, value: T) {
        let next = self.head.take();
        self.head = Some(Box::new(Node { value, next }));
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T: Clone> SinglyLinkedList<T> {
    pub fn to_vec(&self) -> Vec<T> {
        let mut values = Vec::with_capacity(self.len);
        let mut current = self.head.as_deref();
        while let Some(node) = current {
            values.push(node.value.clone());
            current = node.next.as_deref();
        }
        values
    }

    pub fn reversed(&self) -> Self {
        let mut result = Self::new();
        let mut current = self.head.as_deref();
        while let Some(node) = current {
            result.push_front(node.value.clone());
            current = node.next.as_deref();
        }
        result
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push_front(&mut self, value: T) {
        self.items.insert(0, value);
    }

    pub fn push_back(&mut self, value: T) {
        self.items.push(value);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.items.pop()
    }
}

impl<T: Clone> DoublyLinkedList<T> {
    pub fn to_vec(&self) -> Vec<T> {
        self.items.clone()
    }
}

impl<T> CircularLinkedList<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push_back(&mut self, value: T) {
        self.items.push(value);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T: Clone> CircularLinkedList<T> {
    pub fn cycle_walk(&self, steps: usize) -> Vec<T> {
        if self.items.is_empty() {
            return Vec::new();
        }
        let mut out = Vec::with_capacity(steps);
        for step in 0..steps {
            out.push(self.items[step % self.items.len()].clone());
        }
        out
    }
}

pub fn reverse_values(values: &[i32]) -> Vec<i32> {
    values.iter().rev().copied().collect()
}

pub fn detect_cycle(values: &[usize], start: usize) -> bool {
    let mut slow = start;
    let mut fast = start;
    while fast < values.len() && values[fast] < values.len() {
        slow = values[slow];
        fast = values[values[fast]];
        if slow == fast {
            return true;
        }
    }
    false
}

pub fn merge_sorted_lists(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);
    merged
}

pub fn middle_of_list(values: &[i32]) -> Option<i32> {
    values.get(values.len() / 2).copied()
}

#[cfg(test)]
mod tests {
    use super::{
        CircularLinkedList, DoublyLinkedList, SinglyLinkedList, detect_cycle, merge_sorted_lists,
        middle_of_list, reverse_values,
    };

    #[test]
    fn list_push_and_pop_front() {
        let mut list = SinglyLinkedList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn linked_list_variants_and_operations_are_covered() {
        let mut doubly = DoublyLinkedList::new();
        doubly.push_front(2);
        doubly.push_front(1);
        doubly.push_back(3);
        assert_eq!(doubly.to_vec(), vec![1, 2, 3]);

        let mut circular = CircularLinkedList::new();
        circular.push_back(1);
        circular.push_back(2);
        circular.push_back(3);
        assert_eq!(circular.cycle_walk(5), vec![1, 2, 3, 1, 2]);

        assert_eq!(reverse_values(&[1, 2, 3]), vec![3, 2, 1]);
        assert!(detect_cycle(&[1, 2, 0], 0));
        assert_eq!(merge_sorted_lists(&[1, 3, 5], &[2, 4]), vec![1, 2, 3, 4, 5]);
        assert_eq!(middle_of_list(&[1, 2, 3, 4, 5]), Some(3));
    }
}
