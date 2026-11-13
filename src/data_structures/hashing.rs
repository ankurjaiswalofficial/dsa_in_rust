//! Hashing basics.
//!
//! Hash-based structures trade ordering away so lookup, insertion, and deletion
//! can be very fast on average.
//!
//! # Covered topics
//!
//! - Hash table
//! - Hash map
//! - Hash set
//! - Collision handling
//! - Frequency count
//! - Two Sum: brute force O(n^2) vs one-pass hash map O(n) (video: DSA in Rust, episode 3)
//!
//! # Example
//!
//! ```
//! use project1::data_structures::hashing::{frequency_count, hash_set_contains, ChainedHashTable};
//!
//! let mut table = ChainedHashTable::new(8);
//! table.insert("apple".to_string(), 3);
//! assert_eq!(table.get(&"apple".to_string()), Some(&3));
//! assert_eq!(frequency_count("banana").get(&'a'), Some(&3));
//! assert!(hash_set_contains(&["x", "y", "z"], "y"));
//! ```

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct ChainedHashTable<K, V> {
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Eq + Hash, V> ChainedHashTable<K, V> {
    pub fn new(bucket_count: usize) -> Self {
        let buckets = std::iter::repeat_with(Vec::new)
            .take(bucket_count.max(1))
            .collect();
        Self { buckets }
    }

    fn bucket_index(&self, key: &K) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.bucket_index(&key);
        for (existing_key, existing_value) in &mut self.buckets[index] {
            if existing_key == &key {
                *existing_value = value;
                return;
            }
        }
        self.buckets[index].push((key, value));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.bucket_index(key);
        self.buckets[index]
            .iter()
            .find_map(|(existing_key, value)| (existing_key == key).then_some(value))
    }

    pub fn collision_handling_summary() -> &'static str {
        "This table uses separate chaining: each bucket stores multiple key-value pairs, \
         so collisions are handled by grouping entries in the same bucket."
    }
}

pub fn hash_map_example(values: &[(&str, i32)]) -> HashMap<String, i32> {
    values.iter().map(|(key, value)| ((*key).to_string(), *value)).collect()
}

pub fn hash_set_contains(values: &[&str], target: &str) -> bool {
    let set: HashSet<&str> = values.iter().copied().collect();
    set.contains(target)
}

pub fn frequency_count(text: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for ch in text.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts
}

/// Two Sum, brute force: check every pair. O(n^2) time, O(1) extra space.
///
/// The inner loop starts at `i + 1`, so an element is never paired with itself
/// (starting at 0 returns `(0, 0)` for `[3, 3]` with target 6).
pub fn two_sum_brute(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return Some((i, j));
            }
        }
    }
    None
}

/// Two Sum, one pass with a hash map: O(n) time, O(n) extra space.
///
/// For each number `x`, look up its complement `target - x` among the numbers
/// seen so far, and only then remember `x`. Looking up first guarantees `j < i`.
pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::new(); // value -> index
    for (i, &x) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - x)) {
            return Some((j, i));
        }
        seen.insert(x, i);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chained_hash_table_stores_values() {
        let mut table = ChainedHashTable::new(4);
        table.insert("apple".to_string(), 1);
        table.insert("banana".to_string(), 2);
        assert_eq!(table.get(&"apple".to_string()), Some(&1));
        assert!(ChainedHashTable::<String, i32>::collision_handling_summary().contains("separate chaining"));
    }

    #[test]
    fn hash_map_set_and_frequency_examples_work() {
        let map = hash_map_example(&[("x", 1), ("y", 2)]);
        assert_eq!(map.get("y"), Some(&2));
        assert!(hash_set_contains(&["red", "green"], "green"));
        assert_eq!(frequency_count("banana").get(&'a'), Some(&3));
    }

    #[test]
    fn two_sum_brute_and_hash_map_agree() {
        for (nums, target, want) in [
            (vec![2, 7, 11, 15], 9, Some((0, 1))),
            (vec![3, 2, 4], 6, Some((1, 2))),
            (vec![3, 3], 6, Some((0, 1))),
            (vec![1, 2, 3], 100, None),
        ] {
            assert_eq!(two_sum_brute(&nums, target), want);
            assert_eq!(two_sum(&nums, target), want);
        }
    }
}
