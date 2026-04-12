//! Convex hull trick for monotonic queries.
//!
//! This optimizes DP recurrences of the form `dp[i] = min(m_j * x_i + b_j)` when lines are added
//! in sorted slope order and queries come in sorted `x` order.
//!
//! # Example
//!
//! ```
//! use project1::advanced::convex_hull_trick::{Line, MonotonicConvexHullTrick};
//!
//! let mut cht = MonotonicConvexHullTrick::new();
//! cht.add_line(Line { m: 3, b: 0 });
//! cht.add_line(Line { m: 2, b: 1 });
//! cht.add_line(Line { m: 1, b: 3 });
//! assert_eq!(cht.query(2), Some(5));
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub m: i64,
    pub b: i64,
}

impl Line {
    fn value(self, x: i64) -> i64 {
        self.m * x + self.b
    }
}

#[derive(Debug, Clone, Default)]
pub struct MonotonicConvexHullTrick {
    lines: std::collections::VecDeque<Line>,
}

impl MonotonicConvexHullTrick {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_line(&mut self, line: Line) {
        while self.lines.len() >= 2 {
            let last = self.lines[self.lines.len() - 1];
            let second_last = self.lines[self.lines.len() - 2];
            if is_obsolete(second_last, last, line) {
                self.lines.pop_back();
            } else {
                break;
            }
        }
        self.lines.push_back(line);
    }

    pub fn query(&mut self, x: i64) -> Option<i64> {
        while self.lines.len() >= 2 && self.lines[0].value(x) >= self.lines[1].value(x) {
            self.lines.pop_front();
        }
        self.lines.front().map(|line| line.value(x))
    }
}

fn is_obsolete(a: Line, b: Line, c: Line) -> bool {
    (b.b - a.b) as i128 * (b.m - c.m) as i128 >= (c.b - b.b) as i128 * (a.m - b.m) as i128
}

#[cfg(test)]
mod tests {
    use super::{Line, MonotonicConvexHullTrick};

    #[test]
    fn cht_answers_monotonic_queries() {
        let mut cht = MonotonicConvexHullTrick::new();
        cht.add_line(Line { m: 5, b: 0 });
        cht.add_line(Line { m: 3, b: 1 });
        cht.add_line(Line { m: 1, b: 3 });
        assert_eq!(cht.query(1), Some(4));
        assert_eq!(cht.query(2), Some(5));
        assert_eq!(cht.query(5), Some(8));
    }
}
