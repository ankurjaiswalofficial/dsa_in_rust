//! Monotonic stack pattern.
//!
//! A monotonic stack keeps elements in sorted order by popping entries that can
//! no longer influence future answers. This turns many nearest-greater or
//! nearest-smaller problems into linear-time scans.
//!
//! # Example
//!
//! ```
//! use project1::patterns::monotonic_stack::next_greater_elements;
//!
//! assert_eq!(next_greater_elements(&[2, 1, 2, 4, 3]), vec![Some(4), Some(2), Some(4), None, None]);
//! ```

pub fn next_greater_elements(values: &[i64]) -> Vec<Option<i64>> {
    let mut answer = vec![None; values.len()];
    let mut stack: Vec<usize> = Vec::new();

    for (index, &value) in values.iter().enumerate() {
        while let Some(&top) = stack.last() {
            if values[top] < value {
                answer[top] = Some(value);
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(index);
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::next_greater_elements;

    #[test]
    fn monotonic_stack_finds_next_greater_values() {
        assert_eq!(
            next_greater_elements(&[2, 1, 2, 4, 3]),
            vec![Some(4), Some(2), Some(4), None, None]
        );
    }
}
