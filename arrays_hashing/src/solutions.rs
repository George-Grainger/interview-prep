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
    let mut anagrams: Vec<Vec<String>> = vec![];

    'outer: for candidate in strs {
        for seen_anagrams in anagrams.iter_mut() {
            if is_anagram(candidate.to_string(), seen_anagrams[0].to_string()) {
                seen_anagrams.push(candidate.to_string());
                continue 'outer;
            }
        }
        anagrams.push(vec![candidate.to_string()]);
    }

    return anagrams;
}
