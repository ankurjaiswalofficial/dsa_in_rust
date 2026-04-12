//! Binary search on answer.
//!
//! Sometimes the value we want is not stored in an array, but the answer space itself is monotone:
//! once a candidate answer becomes feasible, all larger answers are feasible too, or the reverse.
//! In that case we binary search over the answer domain and use a feasibility checker.
//!
//! The sample problem here is the classic "minimum ship capacity within D days".
//!
//! # Example
//!
//! ```
//! use project1::advanced::searching_topics::binary_search_on_answer::minimum_ship_capacity;
//!
//! assert_eq!(minimum_ship_capacity(&[1, 2, 3, 4, 5], 3), 6);
//! ```

pub fn minimum_ship_capacity(weights: &[i32], days: i32) -> i32 {
    let mut left = *weights.iter().max().unwrap_or(&0);
    let mut right: i32 = weights.iter().sum();
    while left < right {
        let mid = left + (right - left) / 2;
        if can_ship(weights, days, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn can_ship(weights: &[i32], days: i32, capacity: i32) -> bool {
    let mut used_days = 1;
    let mut load = 0;
    for &weight in weights {
        if load + weight > capacity {
            used_days += 1;
            load = 0;
        }
        load += weight;
    }
    used_days <= days
}

#[cfg(test)]
mod tests {
    use super::minimum_ship_capacity;

    #[test]
    fn binary_search_on_answer_finds_the_minimum_feasible_capacity() {
        assert_eq!(minimum_ship_capacity(&[1, 2, 3, 4, 5], 3), 6);
    }
}
