//! Greedy plus heap.
//!
//! A heap is useful when the greedy choice depends on the current best among many active
//! candidates. Meeting-room allocation is a common pattern.
//!
//! # Example
//!
//! ```
//! use project1::advanced::greedy_topics::greedy_plus_heap::minimum_meeting_rooms;
//!
//! let meetings = vec![(0, 30), (5, 10), (15, 20)];
//! assert_eq!(minimum_meeting_rooms(&meetings), 2);
//! ```

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn minimum_meeting_rooms(intervals: &[(i32, i32)]) -> usize {
    let mut intervals = intervals.to_vec();
    intervals.sort_unstable();
    let mut heap = BinaryHeap::new();
    let mut answer = 0;
    for (start, end) in intervals {
        while let Some(&Reverse(top_end)) = heap.peek() {
            if top_end <= start {
                heap.pop();
            } else {
                break;
            }
        }
        heap.push(Reverse(end));
        answer = answer.max(heap.len());
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::minimum_meeting_rooms;

    #[test]
    fn greedy_plus_heap_tracks_active_intervals() {
        let meetings = vec![(0, 30), (5, 10), (15, 20)];
        assert_eq!(minimum_meeting_rooms(&meetings), 2);
    }
}
