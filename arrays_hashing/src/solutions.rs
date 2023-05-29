use std::collections::{HashMap, HashSet};

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen: HashSet<i32> = HashSet::new();
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

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, usize> = HashMap::new();

    for (i, n) in nums.iter().enumerate() {
        match seen.get(n) {
            Some(&p) => return vec![p as i32, i as i32],
            None => seen.insert(target - n, i),
        };
    }

    unreachable!("Result should have been found");
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut anagrams: HashMap<[u8; 26], Vec<String>> = HashMap::with_capacity(strs.len());

    for candidate in strs.into_iter() {
        let mut chars: [u8; 26] = [0; 26];
        candidate.bytes().for_each(|c| {
            chars[(c - b'a') as usize] += 1;
        });

        anagrams
            .entry(chars)
            .and_modify(|seen| seen.push(candidate.clone()))
            .or_insert(vec![candidate]);
    }

    anagrams.into_values().collect()
}