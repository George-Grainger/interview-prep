// Solution for: https://leetcode.com/problems/contains-duplicate/

use std::collections::HashSet;

#[test]
fn test_contains_duplicate() {
    assert!(contains_duplicate(vec![1, 2, 3, 1]));
    assert!(!contains_duplicate(vec![1, 2, 3, 4]));
    assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen: HashSet<i32> = HashSet::with_capacity(nums.len());
    nums.iter().any(|&num| !seen.insert(num))
}
