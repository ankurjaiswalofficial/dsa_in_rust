//! Disjoint set union, also called union-find.
//!
//! Union-find tracks connected components under repeated merge operations. It is one of the core
//! graph building blocks behind Kruskal's MST, offline connectivity, and component counting.
//!
//! This advanced-topic module reuses the shared implementation that already exists in the crate.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graphs::disjoint_set_union_find::DisjointSet;
//!
//! let mut dsu = DisjointSet::new(5);
//! dsu.union(0, 1);
//! dsu.union(1, 2);
//! assert!(dsu.connected(0, 2));
//! assert!(!dsu.connected(0, 4));
//! ```

pub use crate::data_structures::disjoint_set::DisjointSet;

#[cfg(test)]
mod tests {
    use super::DisjointSet;

    #[test]
    fn disjoint_set_union_find_tracks_components() {
        let mut dsu = DisjointSet::new(6);
        assert!(dsu.union(0, 1));
        assert!(dsu.union(1, 2));
        assert!(!dsu.union(0, 2));
        assert!(dsu.connected(0, 2));
        assert_eq!(dsu.component_size(1), 3);
    }
}
