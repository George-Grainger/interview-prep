// Solution for: https://leetcode.com/problems/top-k-frequent-elements/

use std::collections::{BinaryHeap, HashMap};

#[test]
fn test_top_k_frequent() {
    assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    assert_eq!(top_k_frequent(vec![1], 2), vec![1]);
}

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
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
