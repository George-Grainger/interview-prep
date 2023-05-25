use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::<i32>::new();
    nums.iter().any(|&num| !seen.insert(num))
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut chars = [0; 26];

    s.bytes().zip(t.bytes()).for_each(|(u, v)| {
        chars[(u - b'a') as usize] += 1;
        chars[(v - b'a') as usize] -= 1;
    });

    chars.iter().all(|&c| c == 0)
}
