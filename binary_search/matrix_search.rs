// Solution for: https://leetcode.com/problems/search-a-2d-matrix/

use std::cmp::Ordering;

#[macro_export]
macro_rules! nassert {
    ($expr:expr) => {
        assert!(!$expr);
    };
}

#[test]
fn test_search_matrix() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 3;
    assert!(search_matrix(matrix.clone(), target));

    let target = 13;
    nassert!(search_matrix(matrix.clone(), target));
}

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let nums: Vec<i32> = matrix.into_iter().flatten().collect();
    let mut top = nums.len();
    let mut bottom = 0;
    while bottom < top {
        let pivot = (top + bottom) / 2;
        match nums[pivot].cmp(&target) {
            Ordering::Less => bottom = pivot + 1,
            Ordering::Equal => return true,
            Ordering::Greater => top = pivot,
        }
    }
    false
}
