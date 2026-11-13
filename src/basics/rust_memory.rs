//! Rust memory-model basics for data structures and algorithms.
//!
//! DSA explanations written in Rust need one extra layer of clarity: not just
//! what the algorithm does, but how ownership and borrowing shape the design.
//!
//! # Key ideas
//!
//! - Ownership tells us who is responsible for cleaning up a value.
//! - Borrowing lets us inspect or mutate without transferring ownership.
//! - `Vec<T>` is the default growable contiguous container.
//! - `Box<T>` is useful when recursive structures need a stable heap allocation.
//! - `Option<T>` expresses absence directly and appears frequently in trees and lists.
//!
//! # Example
//!
//! ```
//! use project1::basics::rust_memory::{container_reason, ownership_summary};
//!
//! assert!(ownership_summary().contains("owner"));
//! assert!(container_reason("Box").is_some());
//! ```

/// Returns a concise explanation of ownership.
pub fn ownership_summary() -> &'static str {
    "Every value has a single owner. When the owner goes out of scope, Rust \
     drops the value automatically unless ownership was moved elsewhere."
}

/// Returns a concise explanation of borrowing.
pub fn borrowing_summary() -> &'static str {
    "Shared borrows allow read-only access, mutable borrows allow mutation, and \
     Rust prevents aliasing patterns that could create memory unsafety."
}

/// Explains why a common container appears so often in DSA code.
pub fn container_reason(name: &str) -> Option<&'static str> {
    match name {
        "Vec" => Some("Vec provides contiguous storage, fast indexing, and amortized O(1) push."),
        "Box" => Some("Box places a value on the heap so recursive types have a known size."),
        "Option" => Some("Option models the presence or absence of a child, value, or next node."),
        "VecDeque" => Some("VecDeque supports efficient push and pop at both ends."),
        _ => None,
    }
}

/// Stack sizes of common types on this machine (DSA in Rust, episode 5).
///
/// `size_of` reports the bytes a value takes on the stack. A `Vec` or `String`
/// is a pointer, a length and a capacity; its items live on the heap. Measured
/// on 64-bit: i32 4, [i32; 10] 40, Vec 24, String 24, Box 8, Option<Box> 8,
/// &[i32] 16, Option<i32> 8.
///
/// ```
/// use project1::basics::rust_memory::stack_sizes;
///
/// let sizes = stack_sizes();
/// assert_eq!(sizes[0], ("i32", 4));
/// ```
pub fn stack_sizes() -> [(&'static str, usize); 8] {
    use std::mem::size_of;
    [
        ("i32", size_of::<i32>()),
        ("[i32; 10]", size_of::<[i32; 10]>()),
        ("Vec<i32>", size_of::<Vec<i32>>()),
        ("String", size_of::<String>()),
        ("Box<i32>", size_of::<Box<i32>>()),
        ("Option<Box<i32>>", size_of::<Option<Box<i32>>>()),
        ("&[i32]", size_of::<&[i32]>()),
        ("Option<i32>", size_of::<Option<i32>>()),
    ]
}

/// O(1) auxiliary space: one running total, no matter how long the input is.
pub fn sum_constant_space(items: &[i32]) -> i64 {
    items.iter().map(|&x| x as i64).sum()
}

/// O(n) auxiliary space: builds a new `Vec` as long as the input.
pub fn doubled_linear_space(items: &[i32]) -> Vec<i32> {
    items.iter().map(|x| x * 2).collect()
}

/// Sums `0..=n` recursively. Every unfinished call keeps a stack frame, so
/// this uses O(n) stack space: about 1,000,000 calls overflow the 1 MB main
/// thread stack on Windows ("thread 'main' has overflowed its stack").
pub fn sum_recursive(n: u64) -> u64 {
    if n == 0 { 0 } else { n + sum_recursive(n - 1) }
}

/// The same sum as a loop: one running total, O(1) space, any depth.
pub fn sum_iterative(n: u64) -> u64 {
    (1..=n).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summaries_cover_core_rust_concepts() {
        assert!(ownership_summary().contains("owner"));
        assert!(borrowing_summary().contains("mutable borrows"));
    }

    #[test]
    fn known_containers_have_explanations() {
        assert!(container_reason("Vec").unwrap().contains("contiguous"));
        assert!(container_reason("Box").unwrap().contains("recursive"));
        assert!(container_reason("Unknown").is_none());
    }

    #[test]
    fn vec_and_string_are_three_words_on_the_stack() {
        let word = std::mem::size_of::<usize>();
        let sizes = stack_sizes();
        assert_eq!(sizes[1].1, 40);
        assert_eq!(sizes[2].1, 3 * word);
        assert_eq!(sizes[3].1, 3 * word);
        assert_eq!(sizes[4].1, word);
        assert_eq!(sizes[5].1, sizes[4].1, "None is stored as the null pointer");
        assert_eq!(sizes[6].1, 2 * word);
    }

    #[test]
    fn auxiliary_space_examples_agree() {
        let items: Vec<i32> = (1..=1_000).collect();
        assert_eq!(sum_constant_space(&items), 500_500);
        assert_eq!(doubled_linear_space(&items)[999], 2_000);
    }

    #[test]
    fn recursion_and_loop_give_the_same_sum() {
        assert_eq!(sum_recursive(1_000), 500_500);
        assert_eq!(sum_iterative(1_000), 500_500);
        assert_eq!(sum_iterative(1_000_000), 500_000_500_000);
    }
}
