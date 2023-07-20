// Solution for: https://leetcode.com/problems/binary-search/

use std::cmp::Ordering;

#[test]
fn test_binary_search() {
    let input = vec![-1, 0, 3, 5, 9, 12];
    assert_eq!(binary_search(input.clone(), 9), 4);
    assert_eq!(binary_search(input.clone(), 2), -1);
    assert_eq!(binary_search(input.clone(), 13), -1);

    let input = vec![5];
    assert_eq!(binary_search(input.clone(), 5), 0);
    assert_eq!(binary_search(input.clone(), -5), -1);
}

fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut top = nums.len();
    let mut bottom = 0;
    while bottom < top {
        let pivot = (top + bottom) / 2;
        match nums[pivot].cmp(&target) {
            Ordering::Less => bottom = pivot + 1,
            Ordering::Equal => return pivot as i32,
            Ordering::Greater => top = pivot,
        }
    }
    -1
}
