//! Suffix automaton.
//!
//! A suffix automaton compactly represents all substrings of a string. One of its nicest immediate
//! applications is counting distinct substrings.
//!
//! # Example
//!
//! ```
//! use project1::advanced::string_ultra_advanced::suffix_automaton::count_distinct_substrings;
//!
//! assert_eq!(count_distinct_substrings("ababa"), 9);
//! ```

use std::collections::HashMap;

#[derive(Clone, Default)]
struct State {
    len: usize,
    link: Option<usize>,
    next: HashMap<u8, usize>,
}

pub fn count_distinct_substrings(text: &str) -> usize {
    let mut states = vec![State::default()];
    let mut last = 0usize;
    for &byte in text.as_bytes() {
        let current = states.len();
        states.push(State {
            len: states[last].len + 1,
            link: None,
            next: HashMap::new(),
        });
        let mut p = Some(last);
        while let Some(index) = p {
            if states[index].next.contains_key(&byte) {
                break;
            }
            states[index].next.insert(byte, current);
            p = states[index].link;
        }
        if let Some(index) = p {
            let q = states[index].next[&byte];
            if states[index].len + 1 == states[q].len {
                states[current].link = Some(q);
            } else {
                let clone = states.len();
                states.push(State {
                    len: states[index].len + 1,
                    link: states[q].link,
                    next: states[q].next.clone(),
                });
                let mut p = Some(index);
                while let Some(idx) = p {
                    if states[idx].next.get(&byte) != Some(&q) {
                        break;
                    }
                    states[idx].next.insert(byte, clone);
                    p = states[idx].link;
                }
                states[q].link = Some(clone);
                states[current].link = Some(clone);
            }
        } else {
            states[current].link = Some(0);
        }
        last = current;
    }
    states
        .iter()
        .skip(1)
        .map(|state| state.len - states[state.link.expect("linked")].len)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::count_distinct_substrings;

    #[test]
    fn suffix_automaton_counts_distinct_substrings() {
        assert_eq!(count_distinct_substrings("ababa"), 9);
    }
}

