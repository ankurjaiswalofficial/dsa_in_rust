# Rust DSA Encyclopedia

`project1` is a Rust-based data structures and algorithms learning repository.
The codebase is organized as a curriculum, not as random snippets. Each topic is
meant to be read in this order:

1. Explanation
2. Small runnable example
3. Core implementation
4. Tests

If you come back later to revise, you should be able to open a file, understand
the idea quickly, run the example mentally or in code, then inspect the real
implementation and tests.

## How To Use This Repository

For every topic file, follow the same loop:

1. Read the explanation first and make sure the problem statement is clear.
2. Read the basic example to understand the shape of the solution.
3. Study the implementation line by line.
4. Read the tests to see the expected behavior and edge cases.
5. Run `cargo test` after a study session if you want to verify the crate still works.

Recommended rule:
- Do not jump into advanced topics too early.
- Finish one phase before moving to the next.
- Revisit patterns often, because they connect many different problems.

## Recommended Learning Order

This is the suggested progression for a learner starting from fundamentals and
moving toward competitive-programming and interview-level advanced topics.

### Phase 0: Setup And Thinking Style

Start here:
- [src/basics/programming.rs](D:\RustProjects\Project1\src\basics\programming.rs)
- [src/basics/complexity.rs](D:\RustProjects\Project1\src\basics\complexity.rs)
- [src/basics/problem_solving.rs](D:\RustProjects\Project1\src\basics\problem_solving.rs)
- [src/basics/recursion.rs](D:\RustProjects\Project1\src\basics\recursion.rs)

Learn in this order:
1. Basics of programming
2. Recursion
3. Time complexity
4. Space complexity
5. Big O notation
6. Problem-solving mindset

Goal of this phase:
- understand input, output, iteration, functions, recursion, and cost analysis

### Phase 1: Core Linear Structures

Study these next:
- [src/algorithms/arrays.rs](D:\RustProjects\Project1\src\algorithms\arrays.rs)
- [src/algorithms/string_algorithms.rs](D:\RustProjects\Project1\src\algorithms\string_algorithms.rs)
- [src/data_structures/linked_list.rs](D:\RustProjects\Project1\src\data_structures\linked_list.rs)
- [src/data_structures/stack.rs](D:\RustProjects\Project1\src\data_structures\stack.rs)
- [src/data_structures/queue.rs](D:\RustProjects\Project1\src\data_structures\queue.rs)
- [src/data_structures/deque.rs](D:\RustProjects\Project1\src\data_structures\deque.rs)

Suggested order inside this phase:
1. Arrays
2. Strings
3. Linked lists
4. Stack
5. Queue
6. Deque

Topics covered here include:
- array traversal, insertion, deletion, searching
- subarrays, prefix sums, sliding window, two pointers, Kadane, merge intervals, rotate array
- string traversal, palindrome, anagrams, pattern matching, Rabin-Karp, KMP, Z algorithm
- singly, doubly, circular linked list
- reverse list, detect cycle, merge lists, middle of list
- balanced parentheses, infix to postfix/prefix, next greater element, monotonic stack, min stack
- circular queue and priority queue intro

### Phase 2: Fundamental Trees, Hashing, And Heaps

Study these after Phase 1:
- [src/data_structures/binary_tree.rs](D:\RustProjects\Project1\src\data_structures\binary_tree.rs)
- [src/data_structures/binary_search_tree.rs](D:\RustProjects\Project1\src\data_structures\binary_search_tree.rs)
- [src/data_structures/heap.rs](D:\RustProjects\Project1\src\data_structures\heap.rs)
- [src/data_structures/hashing.rs](D:\RustProjects\Project1\src\data_structures\hashing.rs)
- [src/data_structures/trie.rs](D:\RustProjects\Project1\src\data_structures\trie.rs)

Suggested order:
1. Binary tree
2. Binary search tree
3. Heap / priority queue
4. Hashing
5. Trie basics

Topics covered here include:
- tree traversals, level order, height, depth, diameter, LCA, balanced tree, path sum
- min heap, max heap, heapify, heap sort, top K
- hash table, hash map, hash set, collision handling, frequency count

### Phase 3: Standard Algorithms

Now move to the classic algorithm families:
- [src/algorithms/searching.rs](D:\RustProjects\Project1\src\algorithms\searching.rs)
- [src/algorithms/sorting.rs](D:\RustProjects\Project1\src\algorithms\sorting.rs)
- [src/algorithms/graph.rs](D:\RustProjects\Project1\src\algorithms\graph.rs)
- [src/algorithms/greedy.rs](D:\RustProjects\Project1\src\algorithms\greedy.rs)
- [src/algorithms/math.rs](D:\RustProjects\Project1\src\algorithms\math.rs)
- [src/algorithms/bit_manipulation.rs](D:\RustProjects\Project1\src\algorithms\bit_manipulation.rs)

Suggested order:
1. Searching
2. Sorting
3. Graph representation, BFS, DFS
4. Greedy basics
5. Math / number theory basics
6. Bit manipulation basics

Topics covered here include:
- linear search, binary search, binary-search variants
- bubble, selection, insertion, merge, quick, counting, radix, bucket sort
- graph representation, connected components, cycle detection, topological sort, shortest path, bipartite graph
- activity selection, fractional knapsack, Huffman coding, interval scheduling
- primes, sieve, GCD, LCM, modular arithmetic, fast exponentiation
- bitwise operators, set/unset/toggle bits, power of two, XOR, masking

### Phase 4: Core Problem-Solving Patterns

These should be studied alongside the previous phases, but they become much
more useful once arrays and searching feel natural.

Read:
- [src/patterns/prefix_sum.rs](D:\RustProjects\Project1\src\patterns\prefix_sum.rs)
- [src/patterns/sliding_window.rs](D:\RustProjects\Project1\src\patterns\sliding_window.rs)
- [src/patterns/two_pointers.rs](D:\RustProjects\Project1\src\patterns\two_pointers.rs)
- [src/patterns/monotonic_stack.rs](D:\RustProjects\Project1\src\patterns\monotonic_stack.rs)

Goal of this phase:
- stop memorizing problems by name
- start recognizing reusable techniques

### Phase 5: Dynamic Programming And Backtracking

Do this only after recursion, arrays, and complexity are comfortable.

Read:
- [src/algorithms/dynamic_programming.rs](D:\RustProjects\Project1\src\algorithms\dynamic_programming.rs)
- [src/algorithms/backtracking.rs](D:\RustProjects\Project1\src\algorithms\backtracking.rs)

Suggested order:
1. Memoization
2. Tabulation
3. Knapsack
4. LCS
5. LIS
6. Matrix chain multiplication
7. DP on grids
8. Basic DP on trees
9. Subsets and permutations
10. Combination sum
11. N-Queens
12. Sudoku solver

### Phase 6: Intermediate Data Structures

These are extremely useful before starting the advanced folder in depth:
- [src/data_structures/disjoint_set.rs](D:\RustProjects\Project1\src\data_structures\disjoint_set.rs)
- [src/data_structures/fenwick_tree.rs](D:\RustProjects\Project1\src\data_structures\fenwick_tree.rs)
- [src/data_structures/segment_tree.rs](D:\RustProjects\Project1\src\data_structures\segment_tree.rs)

Study order:
1. Disjoint set union
2. Fenwick tree
3. Segment tree

## Advanced Roadmap

Only start this section after the core phases above feel natural.

### Step 1: Advanced Trees, Trie, And Range Queries

Start with:
- [src/advanced/trees](D:\RustProjects\Project1\src\advanced\trees\mod.rs)
- [src/advanced/trie_topics](D:\RustProjects\Project1\src\advanced\trie_topics\mod.rs)
- [src/advanced/range_queries](D:\RustProjects\Project1\src\advanced\range_queries\mod.rs)
- [src/advanced/tree_extensions](D:\RustProjects\Project1\src\advanced\tree_extensions\mod.rs)

Recommended order:
1. AVL tree
2. Red-black tree
3. Fenwick tree and segment tree revision
4. Lazy propagation
5. Order statistic tree
6. Trie insert/search/delete
7. Prefix matching and word search
8. Bitwise trie
9. Euler tour, flattening, binary lifting, LCA
10. Persistent segment tree, merge sort tree, wavelet tree

### Step 2: Advanced Graphs

Then move to:
- [src/advanced/graphs](D:\RustProjects\Project1\src\advanced\graphs\mod.rs)
- [src/advanced/graph_specialized](D:\RustProjects\Project1\src\advanced\graph_specialized\mod.rs)
- [src/advanced/max_flow.rs](D:\RustProjects\Project1\src\advanced\max_flow.rs)

Recommended order:
1. Dijkstra
2. Bellman-Ford
3. Floyd-Warshall
4. MST: Kruskal and Prim
5. SCC
6. Bridges and articulation points
7. Eulerian path / circuit
8. 0-1 BFS
9. Multi-source BFS
10. Binary lifting and HLD
11. Centroid decomposition
12. K shortest paths
13. A*
14. Network flow / Dinic / Edmonds-Karp

### Step 3: Advanced DP, Greedy, And Backtracking

Read:
- [src/advanced/dynamic_programming](D:\RustProjects\Project1\src\advanced\dynamic_programming\mod.rs)
- [src/advanced/greedy_topics](D:\RustProjects\Project1\src\advanced\greedy_topics\mod.rs)
- [src/advanced/backtracking_topics](D:\RustProjects\Project1\src\advanced\backtracking_topics\mod.rs)
- [src/advanced/computational_techniques](D:\RustProjects\Project1\src\advanced\computational_techniques\mod.rs)
- [src/advanced/algorithm_design](D:\RustProjects\Project1\src\advanced\algorithm_design\mod.rs)

Recommended order:
1. DP on strings and subsequences
2. Digit DP
3. Bitmask DP
4. DP with binary search
5. Tree DP
6. State machine DP
7. DP optimizations
8. Greedy plus sorting / heap / intervals / scheduling
9. Branch and bound
10. Constraint solving
11. Divide and conquer
12. Randomized algorithms
13. Recursion tree method
14. Greedy proof techniques, exchange argument, invariants, amortized analysis

### Step 4: Advanced Math, Strings, And Specialized Topics

Read:
- [src/advanced/math_topics](D:\RustProjects\Project1\src\advanced\math_topics\mod.rs)
- [src/advanced/advanced_number_theory](D:\RustProjects\Project1\src\advanced\advanced_number_theory\mod.rs)
- [src/advanced/algebra_polynomial](D:\RustProjects\Project1\src\advanced\algebra_polynomial\mod.rs)
- [src/advanced/strings](D:\RustProjects\Project1\src\advanced\strings\mod.rs)
- [src/advanced/string_ultra_advanced](D:\RustProjects\Project1\src\advanced\string_ultra_advanced\mod.rs)
- [src/advanced/game_theory](D:\RustProjects\Project1\src\advanced\game_theory\mod.rs)
- [src/advanced/practical_topics](D:\RustProjects\Project1\src\advanced\practical_topics\mod.rs)
- [src/advanced/problem_solving_patterns](D:\RustProjects\Project1\src\advanced\problem_solving_patterns\mod.rs)
- [src/advanced/misc_topics](D:\RustProjects\Project1\src\advanced\misc_topics\mod.rs)

Recommended order:
1. Extended Euclid, modular inverse, CRT, Fermat
2. Matrix exponentiation, combinatorics, Catalan
3. Totient, Mobius, inclusion-exclusion, Lucas theorem
4. FFT, NTT, polynomial multiplication
5. Rolling hash, suffix array, LCP, Manacher
6. Suffix tree, Aho-Corasick, suffix automaton
7. Grundy, Nim, Sprague-Grundy
8. Sweep line, coordinate compression, geometry basics, line intersection
9. LRU/LFU cache, iterator design, comparators, edge-case handling
10. Pattern recognition, reduction techniques, transformations, state compression

## If You Want The Shortest Good Path

If you want the most practical order with the least detours, follow this:

1. `basics/programming.rs`
2. `basics/complexity.rs`
3. `basics/recursion.rs`
4. `algorithms/arrays.rs`
5. `algorithms/string_algorithms.rs`
6. `data_structures/stack.rs`
7. `data_structures/queue.rs`
8. `data_structures/linked_list.rs`
9. `algorithms/searching.rs`
10. `algorithms/sorting.rs`
11. `data_structures/binary_tree.rs`
12. `data_structures/binary_search_tree.rs`
13. `data_structures/heap.rs`
14. `data_structures/hashing.rs`
15. `algorithms/graph.rs`
16. `patterns/*`
17. `algorithms/greedy.rs`
18. `algorithms/dynamic_programming.rs`
19. `algorithms/backtracking.rs`
20. `algorithms/bit_manipulation.rs`
21. `algorithms/math.rs`
22. `data_structures/disjoint_set.rs`
23. `data_structures/fenwick_tree.rs`
24. `data_structures/segment_tree.rs`
25. `advanced/*`

## Module Map

Repository structure:

- `src/basics`
  foundation, complexity, recursion, mindset
- `src/data_structures`
  concrete containers and structural operations
- `src/algorithms`
  classic algorithm families
- `src/patterns`
  reusable techniques that appear across many problems
- `src/advanced`
  harder extensions, specialized structures, proof-oriented topics, and deeper problem-solving material

## Running The Project

Run the full test suite:

```powershell
cargo test
```

Build documentation:

```powershell
cargo doc --open
```

## Current Status

The repository currently includes:
- explanations
- runnable examples
- implementations
- tests

The full crate is passing at the moment through `cargo test`.
