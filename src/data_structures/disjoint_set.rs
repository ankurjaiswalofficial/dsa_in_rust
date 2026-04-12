//! Disjoint set union (union-find).
//!
//! This structure answers connectivity questions in near-constant amortized time using path
//! compression and union by size.
//!
//! # Example
//!
//! ```
//! use project1::data_structures::disjoint_set::DisjointSet;
//!
//! let mut dsu = DisjointSet::new(5);
//! dsu.union(0, 1);
//! dsu.union(1, 2);
//! assert!(dsu.connected(0, 2));
//! assert!(!dsu.connected(0, 4));
//! ```

#[derive(Debug, Clone)]
pub struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);
        if root_a == root_b {
            return false;
        }

        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }

        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];
        true
    }

    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn component_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

#[cfg(test)]
mod tests {
    use super::DisjointSet;

    #[test]
    fn union_find_tracks_components() {
        let mut dsu = DisjointSet::new(6);
        assert!(dsu.union(0, 1));
        assert!(dsu.union(1, 2));
        assert!(!dsu.union(0, 2));
        assert!(dsu.connected(0, 2));
        assert_eq!(dsu.component_size(1), 3);
        assert!(!dsu.connected(0, 5));
    }
}
