//! DP optimization with divide and conquer.
//!
//! Full Knuth and divide-and-conquer optimization theory is larger than a single short file, but a
//! good starting point is seeing an actual divide-and-conquer DP optimizer in code.
//!
//! The sample problem partitions an array into `groups` contiguous parts and minimizes the sum of
//! squared segment sums. The optimization computes each DP layer faster than the naive quadratic
//! transition.
//!
//! # Example
//!
//! ```
//! use project1::advanced::dynamic_programming::dp_optimization_knuth_divide_and_conquer::min_partition_cost;
//!
//! assert_eq!(min_partition_cost(&[1, 2, 3, 4], 2), 52);
//! ```

fn segment_cost(prefix: &[i64], left: usize, right: usize) -> i64 {
    let sum = prefix[right] - prefix[left];
    sum * sum
}

pub fn min_partition_cost(values: &[i64], groups: usize) -> i64 {
    let n = values.len();
    let mut prefix = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + values[i];
    }
    let mut previous = vec![i64::MAX / 4; n + 1];
    previous[0] = 0;
    for _ in 0..groups {
        let mut current = vec![i64::MAX / 4; n + 1];
        compute_layer(1, n, 0, n - 1, &mut current, &previous, &prefix);
        previous = current;
    }
    previous[n]
}

fn compute_layer(
    left: usize,
    right: usize,
    opt_left: usize,
    opt_right: usize,
    current: &mut [i64],
    previous: &[i64],
    prefix: &[i64],
) {
    if left > right {
        return;
    }
    let mid = (left + right) / 2;
    let mut best = (i64::MAX / 4, opt_left);
    for split in opt_left..=opt_right.min(mid - 1) {
        let candidate = previous[split] + segment_cost(prefix, split, mid);
        if candidate < best.0 {
            best = (candidate, split);
        }
    }
    current[mid] = best.0;
    if left < mid {
        compute_layer(left, mid - 1, opt_left, best.1, current, previous, prefix);
    }
    if mid < right {
        compute_layer(mid + 1, right, best.1, opt_right, current, previous, prefix);
    }
}

#[cfg(test)]
mod tests {
    use super::min_partition_cost;

    #[test]
    fn divide_and_conquer_dp_optimization_runs_on_a_small_example() {
        assert_eq!(min_partition_cost(&[1, 2, 3, 4], 2), 52);
    }
}
