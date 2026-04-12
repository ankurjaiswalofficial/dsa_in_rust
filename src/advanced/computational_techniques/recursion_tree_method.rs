//! Recursion tree method.
//!
//! The recursion tree method visualizes a recurrence level by level. Summing the work at each
//! level often reveals the asymptotic complexity directly.
//!
//! # Example
//!
//! ```
//! use project1::advanced::computational_techniques::recursion_tree_method::merge_sort_work;
//!
//! assert_eq!(merge_sort_work(8), 32);
//! ```

pub fn merge_sort_work(n: usize) -> usize {
    let mut size = n;
    let mut total = 0;
    while size > 0 {
        total += n;
        size /= 2;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::merge_sort_work;

    #[test]
    fn recursion_tree_counts_total_work_by_levels() {
        assert_eq!(merge_sort_work(8), 32);
    }
}
