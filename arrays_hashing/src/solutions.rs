use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen: HashSet<i32> = HashSet::with_capacity(nums.len());
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

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        counts
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let counts: Vec<(i32, i32)> = counts.into_iter().map(|(a, b)| (b, a)).collect();
    let mut maxheap: BinaryHeap<(i32, i32)> = BinaryHeap::from(counts);

    (0..k)
        .filter_map(|_| maxheap.pop())
        .map(|(_, v)| v)
        .collect()
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![1; nums.len()];

    nums.iter().enumerate().rev().fold(1, |acc, (i, &num)| {
        result[i] = acc;
        acc * num
    });

    nums.iter().enumerate().fold(1, |acc, (i, &num)| {
        result[i] *= acc;
        acc * num
    });

    result
}
