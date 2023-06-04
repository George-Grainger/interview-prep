// Solution for: https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
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
