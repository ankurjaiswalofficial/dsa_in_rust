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
//! - Contains Duplicate and Valid Anagram, three ways each (video: DSA in Rust, episode 9)
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

/// Contains Duplicate, brute force: compare every pair. O(n^2) time, O(1) extra space.
pub fn has_dup_brute(nums: &[i32]) -> bool {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] {
                return true;
            }
        }
    }
    false
}

/// Contains Duplicate by sorting a copy: equal values end up side by side.
/// O(n log n) time, O(n) extra space. `nums.sort()` on a `&[i32]` does not
/// compile (E0596), and taking `&mut [i32]` would reorder the caller's data.
pub fn has_dup_sort(nums: &[i32]) -> bool {
    let mut v = nums.to_vec();
    v.sort();
    v.windows(2).any(|w| w[0] == w[1])
}

/// Contains Duplicate with a hash set: `insert` returns false for a value seen
/// before. O(n) average time, O(n) extra space.
pub fn has_dup_set(nums: &[i32]) -> bool {
    let mut seen = std::collections::HashSet::new();
    nums.iter().any(|&x| !seen.insert(x))
}

/// Valid Anagram by sorting both byte strings. O(n log n) time, O(n) space.
pub fn is_anagram_sort(s: &str, t: &str) -> bool {
    let mut a = s.as_bytes().to_vec();
    let mut b = t.as_bytes().to_vec();
    a.sort_unstable();
    b.sort_unstable();
    a == b
}

/// Valid Anagram with a `[i32; 26]` counting array: O(n) time, O(1) space.
///
/// Only `a..=z` fits the 26 slots: `b'R' - b'a'` would underflow ("attempt to
/// subtract with overflow" in a debug build), so any other byte returns false.
pub fn is_anagram_count(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut count = [0i32; 26];
    for b in s.bytes() {
        if !b.is_ascii_lowercase() {
            return false;
        }
        count[(b - b'a') as usize] += 1;
    }
    for b in t.bytes() {
        if !b.is_ascii_lowercase() {
            return false;
        }
        count[(b - b'a') as usize] -= 1;
    }
    count == [0; 26]
}

/// Valid Anagram for any characters, counting with a `HashMap<char, i32>`.
/// O(n) average time, O(k) space for k distinct characters.
pub fn is_anagram_map(s: &str, t: &str) -> bool {
    let mut count: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    for c in t.chars() {
        *count.entry(c).or_insert(0) -= 1;
    }
    count.values().all(|&n| n == 0)
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

    #[test]
    fn contains_duplicate_three_ways_agree() {
        for f in [has_dup_brute, has_dup_sort, has_dup_set] {
            assert!(f(&[1, 2, 3, 1]));
            assert!(!f(&[1, 2, 3, 4]));
            assert!(!f(&[]));
            assert!(!f(&[7]));
            assert!(f(&[-5, 0, -5]));
        }
        let original = [3, 1, 2];
        has_dup_sort(&original);
        assert_eq!(original, [3, 1, 2]); // the caller's data is not reordered
    }

    #[test]
    fn valid_anagram_three_ways_agree() {
        for f in [is_anagram_sort, is_anagram_count, is_anagram_map] {
            assert!(f("anagram", "nagaram"));
            assert!(!f("rat", "car"));
            assert!(!f("ab", "abc"));
            assert!(f("", ""));
            assert!(!f("Rat", "tar"));
        }
        assert!(is_anagram_map("über", "rübe"));
        assert!(!is_anagram_count("über", "rübe")); // outside a..=z
    }
}
