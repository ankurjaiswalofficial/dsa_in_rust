//! Bridges and articulation points in an undirected graph.
//!
//! Low-link DFS lets us identify fragile parts of an undirected graph:
//! - A `bridge` is an edge whose removal disconnects the graph.
//! - An `articulation point` is a vertex whose removal disconnects the graph.
//!
//! Both can be found in one depth-first traversal.
//!
//! # Example
//!
//! ```
//! use project1::advanced::graphs::bridges_and_articulation_points::bridges_and_articulation_points;
//!
//! let graph = vec![vec![1], vec![0, 2, 3], vec![1], vec![1]];
//! let (bridges, articulation) = bridges_and_articulation_points(&graph);
//! assert_eq!(bridges.len(), 3);
//! assert_eq!(articulation, vec![1]);
//! ```

pub fn bridges_and_articulation_points(graph: &[Vec<usize>]) -> (Vec<(usize, usize)>, Vec<usize>) {
    struct Dfs<'a> {
        graph: &'a [Vec<usize>],
        timer: usize,
        tin: Vec<Option<usize>>,
        low: Vec<usize>,
        bridges: Vec<(usize, usize)>,
        articulation: Vec<bool>,
    }

    impl<'a> Dfs<'a> {
        fn run(&mut self, node: usize, parent: usize) {
            self.tin[node] = Some(self.timer);
            self.low[node] = self.timer;
            self.timer += 1;
            let mut children = 0;
            for &next in &self.graph[node] {
                if next == parent {
                    continue;
                }
                if let Some(entry) = self.tin[next] {
                    self.low[node] = self.low[node].min(entry);
                } else {
                    self.run(next, node);
                    self.low[node] = self.low[node].min(self.low[next]);
                    if self.low[next] > self.tin[node].expect("visited") {
                        self.bridges.push((node, next));
                    }
                    if parent != usize::MAX && self.low[next] >= self.tin[node].expect("visited") {
                        self.articulation[node] = true;
                    }
                    children += 1;
                }
            }
            if parent == usize::MAX && children > 1 {
                self.articulation[node] = true;
            }
        }
    }

    let n = graph.len();
    let mut dfs = Dfs {
        graph,
        timer: 0,
        tin: vec![None; n],
        low: vec![0; n],
        bridges: Vec::new(),
        articulation: vec![false; n],
    };
    for node in 0..n {
        if dfs.tin[node].is_none() {
            dfs.run(node, usize::MAX);
        }
    }
    let articulation = dfs
        .articulation
        .iter()
        .enumerate()
        .filter_map(|(node, &flag)| flag.then_some(node))
        .collect();
    (dfs.bridges, articulation)
}

#[cfg(test)]
mod tests {
    use super::bridges_and_articulation_points;

    #[test]
    fn detects_bridges_and_articulation_points() {
        let graph = vec![vec![1], vec![0, 2, 3], vec![1], vec![1]];
        let (bridges, articulation) = bridges_and_articulation_points(&graph);
        assert_eq!(bridges.len(), 3);
        assert_eq!(articulation, vec![1]);
    }
}
