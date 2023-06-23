// Solution for: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

use std::collections::HashMap;

#[test]
fn test_two_sum_sorted() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2]);
}

fn two_sum_sorted(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(numbers.len());

    for (i, num) in numbers.iter().enumerate() {
        match map.get(num) {
            Some(val) => return vec![(val + 1) as i32, (i + 1) as i32],
            None => map.insert(target - num, i),
        };
    }

    unreachable!("Should be exactly one solution");
}
