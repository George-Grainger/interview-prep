use std::collections::HashMap;
use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::<i32>::new();
    nums.iter().any(|&num| !seen.insert(num))
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut char_map = HashMap::<char, i32>::new();
    for (s_char, t_char) in s.chars().zip(t.chars()) {
        if s_char == t_char {
            continue;
        }

        char_map
            .entry(s_char)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        char_map
            .entry(t_char)
            .and_modify(|count| *count -= 1)
            .or_insert(-1);
    }

    char_map.values().all(|&count| count == 0)
}
