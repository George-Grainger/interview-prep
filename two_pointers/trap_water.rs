// Solution for: https://leetcode.com/problems/trapping-rain-water/

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
    if height.is_empty() {
        return 0;
    }

    let mut lp = 0;
    let mut rp = height.len() - 1;
    let mut left_max = height[lp];
    let mut right_max = height[rp];
    let mut total = 0;

    while lp < rp {
        if left_max < right_max {
            lp += 1;
            left_max = cmp::max(left_max, height[lp]);
            total += left_max - height[lp];
        } else {
            rp -= 1;
            right_max = cmp::max(right_max, height[rp]);
            total += right_max - height[rp];
        }
    }

    total
}
