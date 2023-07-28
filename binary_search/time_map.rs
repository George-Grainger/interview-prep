// Solution for: https://leetcode.com/problems/time-based-key-value-store/

use std::cmp::Ordering;
use std::collections::HashMap;

#[test]
fn test_time_map() {
    let mut tm = TimeMap::new();
    tm.set(String::from("foo"), String::from("bar"), 1);
    assert_eq!(tm.get(String::from("foo"), 1), String::from("bar"));
    assert_eq!(tm.get(String::from("foo"), 3), String::from("bar"));
    tm.set(String::from("foo"), String::from("bar2"), 4);
    assert_eq!(tm.get(String::from("foo"), 4), String::from("bar2"));
    assert_eq!(tm.get(String::from("foo"), 5), String::from("bar2"));

    let mut tm = TimeMap::new();
    tm.set(String::from("love"), String::from("high"), 10);
    tm.set(String::from("love"), String::from("low"), 20);
    assert_eq!(tm.get(String::from("love"), 5), String::from(""));
    assert_eq!(tm.get(String::from("love"), 10), String::from("high"));
    assert_eq!(tm.get(String::from("love"), 15), String::from("high"));
    assert_eq!(tm.get(String::from("love"), 20), String::from("low"));
    assert_eq!(tm.get(String::from("love"), 25), String::from("low"));
}

struct TimeValue {
    timestamps: Vec<i32>,
    values: Vec<String>,
}

struct TimeMap {
    items: HashMap<String, TimeValue>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            items: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        match self.items.get_mut(&key) {
            Some(tv) => {
                tv.values.push(value);
                tv.timestamps.push(timestamp);
            }
            None => {
                let tv = TimeValue {
                    values: vec![value],
                    timestamps: vec![timestamp],
                };
                self.items.insert(key, tv);
            }
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if !self.items.contains_key(&key) {
            return String::from("");
        }

        let tv = self.items.get(&key).unwrap();
        let mut left = 0;
        let mut right = tv.timestamps.len();

        while left < right {
            let pivot = (left + right) / 2;

            match tv.timestamps[pivot].cmp(&timestamp) {
                Ordering::Less => left = pivot + 1,
                Ordering::Equal => return tv.values[pivot].clone(),
                Ordering::Greater => right = pivot,
            }
        }

        if left == 0 {
            String::from("")
        } else {
            tv.values[left - 1].clone()
        }
    }
}
