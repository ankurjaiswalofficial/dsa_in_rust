//! Union by rank.
//!
//! Union by rank attaches the shallower tree under the deeper tree, limiting the growth of tree
//! height before path compression is even applied.
//!
//! # Example
//!
//! ```
//! use project1::advanced::disjoint_set_advanced::union_by_rank::RankDisjointSet;
//!
//! let mut dsu = RankDisjointSet::new(4);
//! dsu.union(0, 1);
//! dsu.union(2, 3);
//! dsu.union(1, 2);
//! assert!(dsu.connected(0, 3));
//! ```

#[derive(Debug, Clone)]
pub struct RankDisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl RankDisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
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
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return false;
        }
        if self.rank[a] < self.rank[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        if self.rank[a] == self.rank[b] {
            self.rank[a] += 1;
        }
        true
    }

    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

#[cfg(test)]
mod tests {
    use super::RankDisjointSet;

    #[test]
    fn union_by_rank_maintains_connectivity() {
        let mut dsu = RankDisjointSet::new(4);
        dsu.union(0, 1);
        dsu.union(2, 3);
        dsu.union(1, 2);
        assert!(dsu.connected(0, 3));
    }
}

