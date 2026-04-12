//! Path compression deep understanding.
//!
//! Path compression rewrites every visited node on a `find` path to point directly at the set
//! representative. The first call may be long, but future calls become almost constant-time.
//!
//! # Example
//!
//! ```
//! use project1::advanced::disjoint_set_advanced::path_compression_deep_understanding::compress_path;
//!
//! let mut parent = vec![0, 0, 1, 2];
//! assert_eq!(compress_path(&mut parent, 3), 0);
//! assert_eq!(parent[3], 0);
//! ```

pub fn compress_path(parent: &mut [usize], node: usize) -> usize {
    if parent[node] != node {
        let root = compress_path(parent, parent[node]);
        parent[node] = root;
    }
    parent[node]
}

#[cfg(test)]
mod tests {
    use super::compress_path;

    #[test]
    fn path_compression_flattens_the_parent_chain() {
        let mut parent = vec![0, 0, 1, 2];
        assert_eq!(compress_path(&mut parent, 3), 0);
        assert_eq!(parent[3], 0);
    }
}

