//! Advanced syllabus overview.
//!
//! This module intentionally documents topics that belong in later phases so the curriculum stays
//! explicit without pretending that a rushed implementation would be good enough.
//!
//! # Example
//!
//! ```
//! use project1::advanced::syllabus::{future_topics, grouped_future_topics};
//!
//! assert!(future_topics().contains(&"Heavy Light Decomposition (HLD)"));
//! assert!(grouped_future_topics()
//!     .iter()
//!     .any(|section| section.category == "Graph (Hard / Specialized)"));
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyllabusSection {
    pub category: &'static str,
    pub topics: &'static [&'static str],
}

const GROUPED_TOPICS: &[SyllabusSection] = &[
    SyllabusSection {
        category: "Computational Techniques / Paradigms",
        topics: &[
            "Divide and Conquer",
            "Meet in the Middle",
            "Randomized Algorithms",
            "Backtracking vs Branch & Bound",
            "Recursion Tree Method",
        ],
    },
    SyllabusSection {
        category: "Advanced Algorithm Design",
        topics: &[
            "Greedy Proof Techniques",
            "Exchange Argument",
            "Invariants",
            "Amortized Analysis (Advanced)",
        ],
    },
    SyllabusSection {
        category: "Disjoint Set Advanced",
        topics: &[
            "Union by Rank",
            "Path Compression (deep understanding)",
            "DSU with rollback",
        ],
    },
    SyllabusSection {
        category: "Graph (Hard / Specialized)",
        topics: &[
            "0-1 BFS",
            "Multi-source BFS",
            "K Shortest Paths",
            "A* Algorithm",
            "Heavy Light Decomposition (HLD)",
            "Centroid Decomposition",
            "Binary Lifting",
        ],
    },
    SyllabusSection {
        category: "Tree Advanced Extensions",
        topics: &[
            "Euler Tour Technique",
            "Tree Flattening",
            "Re-rooting Technique",
            "LCA (Binary Lifting, Euler Tour + RMQ)",
        ],
    },
    SyllabusSection {
        category: "Range Query + Query Optimization",
        topics: &[
            "Persistent Segment Tree",
            "Merge Sort Tree",
            "Wavelet Tree",
        ],
    },
    SyllabusSection {
        category: "Advanced Number Theory",
        topics: &[
            "Euler Totient Function",
            "Möbius Function",
            "Inclusion-Exclusion Principle",
            "Lucas Theorem",
        ],
    },
    SyllabusSection {
        category: "Algebra / Polynomial",
        topics: &[
            "Fast Fourier Transform (FFT)",
            "Number Theoretic Transform (NTT)",
            "Polynomial Multiplication",
        ],
    },
    SyllabusSection {
        category: "String Ultra Advanced",
        topics: &[
            "Aho-Corasick Algorithm",
            "Suffix Automaton",
        ],
    },
    SyllabusSection {
        category: "Game Theory (Important for CP)",
        topics: &[
            "Grundy Numbers",
            "Nim Game",
            "Sprague-Grundy Theorem",
        ],
    },
    SyllabusSection {
        category: "Misc High-Level Concepts",
        topics: &[
            "Bitset Optimization",
            "Coordinate Geometry Basics",
            "Sweep Line Algorithm",
            "Line Intersection Algorithms",
        ],
    },
    SyllabusSection {
        category: "Practical / Interview-Oriented Additions",
        topics: &[
            "Design Data Structures (LRU Cache, LFU Cache)",
            "Iterator Design",
            "Custom Comparators",
            "Handling Edge Cases systematically",
        ],
    },
    SyllabusSection {
        category: "Problem-Solving Patterns",
        topics: &[
            "Pattern Recognition",
            "Reduction Techniques",
            "Transformations (array -> graph, etc.)",
            "State Compression",
        ],
    },
    SyllabusSection {
        category: "Additional Advanced Topics",
        topics: &[
            "AVL Tree",
            "Red-Black Tree",
            "Lazy Propagation",
            "Fenwick Tree (Binary Indexed Tree)",
            "Order Statistic Tree",
            "Trie Basics",
            "Insert / Search / Delete",
            "Prefix Matching",
            "Word Search",
            "Bitwise Trie",
            "Dijkstra Algorithm",
            "Bellman-Ford Algorithm",
            "Floyd-Warshall Algorithm",
            "Minimum Spanning Tree (Kruskal, Prim)",
            "Disjoint Set (Union-Find)",
            "Strongly Connected Components (Kosaraju, Tarjan)",
            "Bridges & Articulation Points",
            "Eulerian Path / Circuit",
            "Network Flow (Ford-Fulkerson, Edmonds-Karp)",
            "Advanced matching algorithms",
            "DP on Subsequences",
            "DP on Strings",
            "Digit DP",
            "Bitmask DP",
            "DP with Binary Search",
            "DP Optimization (Knuth, Divide & Conquer)",
            "Tree DP (Advanced)",
            "State Machine DP",
            "Greedy with Sorting",
            "Greedy + Heap",
            "Interval Problems (Advanced)",
            "Scheduling Problems",
            "Advanced Constraint Solving",
            "Optimization Problems",
            "Extended Euclidean Algorithm",
            "Modular Inverse",
            "Chinese Remainder Theorem",
            "Fermat's Little Theorem",
            "Matrix Exponentiation",
            "Combinatorics (nCr, nPr)",
            "Catalan Numbers",
            "Subset Generation using Bits",
            "Gray Code",
            "Advanced XOR Techniques",
            "Ternary Search",
            "Binary Search on Answer",
            "Exponential Search",
            "Suffix Array",
            "LCP Array",
            "Suffix Tree",
            "Rolling Hash",
            "Manacher's Algorithm",
            "Sparse Table",
            "Mo's Algorithm",
            "Sliding Window Advanced",
            "Two Pointer Advanced",
            "Offline Queries",
            "Coordinate Compression",
            "B-trees",
            "B+ trees",
        ],
    },
];

pub fn grouped_future_topics() -> &'static [SyllabusSection] {
    GROUPED_TOPICS
}

pub fn future_topics() -> Vec<&'static str> {
    GROUPED_TOPICS
        .iter()
        .flat_map(|section| section.topics.iter().copied())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{future_topics, grouped_future_topics};

    #[test]
    fn future_syllabus_lists_next_phase_topics() {
        let topics = future_topics();
        assert!(topics.contains(&"Heavy Light Decomposition (HLD)"));
        assert!(topics.contains(&"Fast Fourier Transform (FFT)"));
        assert!(topics.contains(&"AVL Tree"));
        assert!(topics.contains(&"Coordinate Compression"));
        assert!(topics.contains(&"State Compression"));
        assert!(topics.contains(&"DSU with rollback"));
    }

    #[test]
    fn future_syllabus_keeps_grouped_categories() {
        let sections = grouped_future_topics();
        assert!(sections
            .iter()
            .any(|section| section.category == "Graph (Hard / Specialized)"));
        assert!(sections
            .iter()
            .any(|section| section.topics.contains(&"Network Flow (Ford-Fulkerson, Edmonds-Karp)")));
        assert!(sections
            .iter()
            .any(|section| section.category == "Problem-Solving Patterns"));
    }
}
