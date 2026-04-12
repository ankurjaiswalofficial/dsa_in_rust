//! Greedy algorithms.
//!
//! Greedy methods make the best-looking local decision at each step. They are
//! powerful when the problem has an exchange argument or other structure that
//! proves local optimal choices compose into a global optimum.
//!
//! # Example
//!
//! ```
//! use project1::algorithms::greedy::{activity_selection, minimum_coins_canonical};
//!
//! let intervals = vec![(1, 3), (2, 5), (4, 7), (6, 9), (8, 10)];
//! assert_eq!(activity_selection(&intervals), vec![(1, 3), (4, 7), (8, 10)]);
//! assert_eq!(minimum_coins_canonical(&[25, 10, 5, 1], 37), Some(vec![25, 10, 1, 1]));
//! ```

pub fn activity_selection(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut intervals = intervals.to_vec();
    intervals.sort_by_key(|&(_, end)| end);

    let mut chosen = Vec::new();
    let mut last_end = i32::MIN;
    for interval @ (start, end) in intervals {
        if start >= last_end {
            chosen.push(interval);
            last_end = end;
        }
    }
    chosen
}

pub fn minimum_coins_canonical(coins_desc: &[i32], mut amount: i32) -> Option<Vec<i32>> {
    let mut answer = Vec::new();
    for &coin in coins_desc {
        while amount >= coin {
            amount -= coin;
            answer.push(coin);
        }
    }
    (amount == 0).then_some(answer)
}

pub fn interval_scheduling(intervals: &[(i32, i32)]) -> usize {
    activity_selection(intervals).len()
}

pub fn fractional_knapsack(items: &[(f64, f64)], capacity: f64) -> f64 {
    let mut items = items.to_vec();
    items.sort_by(|&(w1, v1), &(w2, v2)| {
        (v2 / w2)
            .partial_cmp(&(v1 / w1))
            .expect("weights are positive and finite")
    });

    let mut remaining = capacity;
    let mut total = 0.0;
    for (weight, value) in items {
        if remaining <= 0.0 {
            break;
        }
        let take = remaining.min(weight);
        total += value * (take / weight);
        remaining -= take;
    }
    total
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HuffmanNode {
    pub frequency: usize,
    pub symbol: Option<char>,
    pub left: Option<Box<HuffmanNode>>,
    pub right: Option<Box<HuffmanNode>>,
}

pub fn huffman_tree(frequencies: &[(char, usize)]) -> Option<HuffmanNode> {
    if frequencies.is_empty() {
        return None;
    }
    let mut nodes: Vec<HuffmanNode> = frequencies
        .iter()
        .map(|&(symbol, frequency)| HuffmanNode {
            frequency,
            symbol: Some(symbol),
            left: None,
            right: None,
        })
        .collect();

    while nodes.len() > 1 {
        nodes.sort_by_key(|node| node.frequency);
        let left = Box::new(nodes.remove(0));
        let right = Box::new(nodes.remove(0));
        nodes.push(HuffmanNode {
            frequency: left.frequency + right.frequency,
            symbol: None,
            left: Some(left),
            right: Some(right),
        });
    }
    nodes.pop()
}

#[cfg(test)]
mod tests {
    use super::{
        activity_selection, fractional_knapsack, huffman_tree, interval_scheduling,
        minimum_coins_canonical,
    };

    #[test]
    fn greedy_activity_selection_prefers_earliest_finish() {
        let intervals = vec![(1, 3), (2, 5), (4, 7), (6, 9), (8, 10)];
        assert_eq!(activity_selection(&intervals), vec![(1, 3), (4, 7), (8, 10)]);
    }

    #[test]
    fn greedy_coin_change_works_for_canonical_systems() {
        assert_eq!(minimum_coins_canonical(&[25, 10, 5, 1], 37), Some(vec![25, 10, 1, 1]));
        assert_eq!(minimum_coins_canonical(&[4, 3], 2), None);
    }

    #[test]
    fn more_greedy_topics_are_covered() {
        let intervals = vec![(1, 3), (2, 5), (4, 7), (8, 10)];
        assert_eq!(interval_scheduling(&intervals), 3);
        assert!((fractional_knapsack(&[(10.0, 60.0), (20.0, 100.0), (30.0, 120.0)], 50.0) - 240.0).abs() < 1e-9);

        let tree = huffman_tree(&[('a', 5), ('b', 9), ('c', 12)]).expect("tree exists");
        assert_eq!(tree.frequency, 26);
    }
}
