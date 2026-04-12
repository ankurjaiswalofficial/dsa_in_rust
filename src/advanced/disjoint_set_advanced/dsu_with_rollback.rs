//! DSU with rollback.
//!
//! Rollback DSU supports undoing unions, which is useful in offline dynamic connectivity and
//! divide-and-conquer-on-time techniques.
//!
//! # Example
//!
//! ```
//! use project1::advanced::disjoint_set_advanced::dsu_with_rollback::RollbackDisjointSet;
//!
//! let mut dsu = RollbackDisjointSet::new(3);
//! let checkpoint = dsu.snapshot();
//! dsu.union(0, 1);
//! assert!(dsu.connected(0, 1));
//! dsu.rollback(checkpoint);
//! assert!(!dsu.connected(0, 1));
//! ```

#[derive(Debug, Clone)]
pub struct RollbackDisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
    history: Vec<(usize, usize, usize)>,
}

impl RollbackDisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            history: Vec::new(),
        }
    }

    pub fn snapshot(&self) -> usize {
        self.history.len()
    }

    pub fn rollback(&mut self, snapshot: usize) {
        while self.history.len() > snapshot {
            let (child, parent, parent_size) = self.history.pop().expect("history exists");
            self.parent[child] = child;
            self.size[parent] = parent_size;
        }
    }

    pub fn find(&self, mut x: usize) -> usize {
        while self.parent[x] != x {
            x = self.parent[x];
        }
        x
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.history.push((b, a, self.size[a]));
        self.parent[b] = a;
        self.size[a] += self.size[b];
        true
    }

    pub fn connected(&self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

#[cfg(test)]
mod tests {
    use super::RollbackDisjointSet;

    #[test]
    fn rollback_dsu_can_undo_unions() {
        let mut dsu = RollbackDisjointSet::new(3);
        let checkpoint = dsu.snapshot();
        dsu.union(0, 1);
        assert!(dsu.connected(0, 1));
        dsu.rollback(checkpoint);
        assert!(!dsu.connected(0, 1));
    }
}
