//! Binary min-heap.
//!
//! A heap keeps the smallest element at the root while remaining a complete
//! binary tree. This makes it ideal for repeatedly extracting the minimum while
//! supporting incremental insertion.
//!
//! # Why this structure exists
//!
//! Fully sorting data is unnecessary when you only need the next best item
//! repeatedly. A heap gives `O(log n)` insertion and removal of the minimum
//! while using simple array storage.
//!
//! # Example
//!
//! ```
//! use project1::data_structures::heap::{top_k_largest, MinHeap};
//!
//! let mut heap = MinHeap::new();
//! heap.push(5);
//! heap.push(2);
//! heap.push(7);
//! assert_eq!(heap.peek(), Some(&2));
//! assert_eq!(heap.pop(), Some(2));
//! assert_eq!(top_k_largest(&[5, 1, 9, 3, 7], 2), vec![9, 7]);
//! ```

#[derive(Debug, Clone, Default)]
pub struct MinHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.sift_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let result = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        result
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn sift_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.data[index] < self.data[parent] {
                self.data.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        let len = self.data.len();
        loop {
            let left = index * 2 + 1;
            let right = index * 2 + 2;
            let mut smallest = index;

            if left < len && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < len && self.data[right] < self.data[smallest] {
                smallest = right;
            }
            if smallest == index {
                break;
            }
            self.data.swap(index, smallest);
            index = smallest;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MaxHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        let mut index = self.data.len() - 1;
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.data[index] > self.data[parent] {
                self.data.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let result = self.data.pop();
        let len = self.data.len();
        let mut index = 0;
        while index < len {
            let left = index * 2 + 1;
            let right = index * 2 + 2;
            let mut largest = index;
            if left < len && self.data[left] > self.data[largest] {
                largest = left;
            }
            if right < len && self.data[right] > self.data[largest] {
                largest = right;
            }
            if largest == index {
                break;
            }
            self.data.swap(index, largest);
            index = largest;
        }
        result
    }
}

pub fn heapify_min<T: Ord + Clone>(values: &[T]) -> MinHeap<T> {
    let mut heap = MinHeap::new();
    for value in values.iter().cloned() {
        heap.push(value);
    }
    heap
}

pub fn top_k_largest(values: &[i32], k: usize) -> Vec<i32> {
    let mut heap = MaxHeap::new();
    for &value in values {
        heap.push(value);
    }
    let mut answer = Vec::new();
    for _ in 0..k.min(values.len()) {
        if let Some(value) = heap.pop() {
            answer.push(value);
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::{MaxHeap, MinHeap, heapify_min, top_k_largest};

    #[test]
    fn heap_preserves_minimum_order() {
        let mut heap = MinHeap::new();
        for value in [5, 1, 4, 2, 3] {
            heap.push(value);
        }

        let mut popped = Vec::new();
        while let Some(value) = heap.pop() {
            popped.push(value);
        }

        assert_eq!(popped, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn max_heap_heapify_and_top_k_work() {
        let mut heap = MaxHeap::new();
        for value in [5, 1, 4, 2, 3] {
            heap.push(value);
        }
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(4));

        let mut min_heap = heapify_min(&[4, 1, 3]);
        assert_eq!(min_heap.pop(), Some(1));
        assert_eq!(top_k_largest(&[5, 1, 9, 3, 7], 3), vec![9, 7, 5]);
    }
}
