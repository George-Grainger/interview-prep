// Solution for: https://leetcode.com/problems/trapping-rain-water/submissions/

use std::cmp;

#[test]
fn test_trap_water() {
    let blocks = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    assert_eq!(trap_water(blocks), 6);
    let blocks = vec![4, 2, 0, 3, 2, 5];
    assert_eq!(trap_water(blocks), 9);
    let blocks = vec![0, 1, 0];
    assert_eq!(trap_water(blocks), 0);
    let blocks = vec![2, 0, 2];
    assert_eq!(trap_water(blocks), 2);
    let blocks = vec![5, 5, 1, 7, 1, 1, 5, 2, 7, 6];
    assert_eq!(trap_water(blocks), 23);
}

fn trap_water(height: Vec<i32>) -> i32 {
    let mut max_index: usize = 0;
    let mut max_block: i32 = 0;
    let mut increment: i32 = 0;
    let mut total: i32 = 0;

    for (i, &block) in height.iter().enumerate() {
        if block >= max_block {
            max_block = block;
            max_index = i;
            total += increment;
            increment = 0;
        } else {
            increment += max_block - block;
        }
    }

    if increment > 0 {
        max_block = 0;
        for &block in height[max_index..].into_iter().rev() {
            max_block = cmp::max(block, max_block);
            total += max_block - block;
        }
    }

    total
}
