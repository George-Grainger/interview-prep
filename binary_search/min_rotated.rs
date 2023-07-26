// Solution for: https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

#[test]
fn test_find_min() {
    let nums = vec![3, 4, 5, 1, 2];
    assert_eq!(find_min(nums), 1);

    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    assert_eq!(find_min(nums), 0);

    let nums = vec![11, 13, 15, 17];
    assert_eq!(find_min(nums), 11);

    let nums = vec![2, 1];
    assert_eq!(find_min(nums), 1);
}

fn find_min(nums: Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let middle = (right + left) / 2;
        if nums[middle] > nums[right] {
            left = middle + 1;
        } else {
            right = middle;
        }
    }
    nums[left]
}
