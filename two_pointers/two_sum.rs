// Solution for: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

use std::cmp::Ordering;

#[test]
fn test_two_sum_sorted() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2]);
}

fn two_sum_sorted(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = numbers.len() - 1;

    while i < j {
        match (numbers[i] + numbers[j]).cmp(&target) {
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
            Ordering::Equal => return vec![(i + 1) as i32, (j + 1) as i32],
        }
    }
    unreachable!("Should be exactly one solution");
}
