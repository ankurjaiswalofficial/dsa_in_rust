//! Meet in the middle.
//!
//! Meet in the middle splits an exponential search into two smaller exponential searches and then
//! combines the results. It is especially useful for subset-sum style problems around `n = 40`.
//!
//! # Example
//!
//! ```
//! use project1::advanced::misc_topics::meet_in_the_middle::max_subset_sum_not_exceeding;
//!
//! assert_eq!(max_subset_sum_not_exceeding(&[3, 34, 4, 12, 5, 2], 10), 10);
//! ```

pub fn max_subset_sum_not_exceeding(values: &[i64], limit: i64) -> i64 {
    let mid = values.len() / 2;
    let left = subset_sums(&values[..mid]);
    let mut right = subset_sums(&values[mid..]);
    right.sort_unstable();
    let mut answer = 0;
    for sum in left {
        if sum > limit {
            continue;
        }
        let remaining = limit - sum;
        let index = right.partition_point(|&value| value <= remaining);
        if index > 0 {
            answer = answer.max(sum + right[index - 1]);
        }
    }
    answer
}

fn subset_sums(values: &[i64]) -> Vec<i64> {
    let mut sums = Vec::new();
    for mask in 0..(1usize << values.len()) {
        let mut total = 0;
        for (index, &value) in values.iter().enumerate() {
            if (mask >> index) & 1 == 1 {
                total += value;
            }
        }
        sums.push(total);
    }
    sums
}

#[cfg(test)]
mod tests {
    use super::max_subset_sum_not_exceeding;

    #[test]
    fn meet_in_the_middle_solves_subset_sum_style_queries() {
        assert_eq!(max_subset_sum_not_exceeding(&[3, 34, 4, 12, 5, 2], 10), 10);
    }
}
