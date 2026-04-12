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
}
