use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::<i32>::new();
    nums.iter().any(|&num| !seen.insert(num))
}
