//! Advanced amortized analysis.
//!
//! Amortized analysis bounds the average cost per operation over a whole sequence, even when some
//! individual operations are expensive.
//!
//! The function below computes the total copy work done by repeated dynamic-array growth with
//! doubling, showing why `push` has `O(1)` amortized cost.
//!
//! # Example
//!
//! ```
//! use project1::advanced::algorithm_design::amortized_analysis_advanced::doubling_resize_copy_cost;
//!
//! assert_eq!(doubling_resize_copy_cost(8), 7);
//! ```

pub fn doubling_resize_copy_cost(final_size: usize) -> usize {
    let mut capacity = 1usize;
    let mut copied = 0usize;
    while capacity < final_size {
        copied += capacity;
        capacity *= 2;
    }
    copied
}

#[cfg(test)]
mod tests {
    use super::doubling_resize_copy_cost;

    #[test]
    fn amortized_analysis_counts_total_resize_work() {
        assert_eq!(doubling_resize_copy_cost(8), 7);
    }
}

