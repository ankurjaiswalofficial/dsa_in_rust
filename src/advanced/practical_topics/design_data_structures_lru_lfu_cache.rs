//! Design data structures: LRU and LFU cache.
//!
//! This file includes simple LRU and LFU cache implementations, which are common interview-oriented
//! data structures combining a hashmap with eviction policy tracking.
//!
//! # Example
//!
//! ```
//! use project1::advanced::practical_topics::design_data_structures_lru_lfu_cache::LruCache;
//!
//! let mut cache = LruCache::new(2);
//! cache.put(1, 10);
//! cache.put(2, 20);
//! assert_eq!(cache.get(1), Some(10));
//! cache.put(3, 30);
//! assert_eq!(cache.get(2), None);
//! ```

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct LruCache {
    capacity: usize,
    values: HashMap<i32, i32>,
    order: VecDeque<i32>,
}

impl LruCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            values: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        let value = *self.values.get(&key)?;
        self.touch(key);
        Some(value)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.values.contains_key(&key) {
            self.values.insert(key, value);
            self.touch(key);
            return;
        }
        if self.values.len() == self.capacity {
            if let Some(oldest) = self.order.pop_front() {
                self.values.remove(&oldest);
            }
        }
        self.values.insert(key, value);
        self.order.push_back(key);
    }

    fn touch(&mut self, key: i32) {
        if let Some(position) = self.order.iter().position(|&candidate| candidate == key) {
            self.order.remove(position);
        }
        self.order.push_back(key);
    }
}

#[derive(Debug, Clone)]
pub struct LfuCache {
    capacity: usize,
    time: usize,
    values: HashMap<i32, (i32, usize, usize)>,
}

impl LfuCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            time: 0,
            values: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        let (value, freq, stamp) = self.values.get_mut(&key)?;
        *freq += 1;
        self.time += 1;
        *stamp = self.time;
        Some(*value)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        if let Some((stored, freq, stamp)) = self.values.get_mut(&key) {
            *stored = value;
            *freq += 1;
            self.time += 1;
            *stamp = self.time;
            return;
        }
        if self.values.len() == self.capacity {
            let victim = self
                .values
                .iter()
                .min_by_key(|(_, entry)| {
                    let (_, freq, stamp) = **entry;
                    (freq, stamp)
                })
                .map(|(&key, _)| key)
                .expect("cache is non-empty");
            self.values.remove(&victim);
        }
        self.time += 1;
        self.values.insert(key, (value, 1, self.time));
    }
}

#[cfg(test)]
mod tests {
    use super::{LfuCache, LruCache};

    #[test]
    fn lru_cache_evicts_the_least_recently_used_key() {
        let mut cache = LruCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.get(1), Some(10));
        cache.put(3, 30);
        assert_eq!(cache.get(2), None);
    }

    #[test]
    fn lfu_cache_evicts_the_least_frequently_used_key() {
        let mut cache = LfuCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.get(1), Some(10));
        cache.put(3, 30);
        assert_eq!(cache.get(2), None);
        assert_eq!(cache.get(3), Some(30));
    }
}
