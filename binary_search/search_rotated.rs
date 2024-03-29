// Solution for: https://leetcode.com/problems/search-in-rotated-sorted-array/

#[test]
fn test_search_rotated() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 0;
    assert_eq!(search_rotated(nums, target), 4);

    let nums = vec![1];
    let target = 0;
    assert_eq!(search_rotated(nums.clone(), target), -1);

    let target = 1;
    assert_eq!(search_rotated(nums.clone(), target), 0);

    let nums = vec![5, 1, 3];
    let target = 5;
    assert_eq!(search_rotated(nums.clone(), target), 0);

    let target = 3;
    assert_eq!(search_rotated(nums.clone(), target), 2);

    let nums = vec![1, 3, 5];
    let target = 5;
    assert_eq!(search_rotated(nums.clone(), target), 2);

    let nums = vec![7, 8, 1, 2, 3, 4, 5, 6];
    let target = 2;
    assert_eq!(search_rotated(nums, target), 3);
}

fn search_rotated(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let middle = (left + right) / 2;
        if nums[middle] == target {
            return middle as i32;
        }

        if (nums[middle] >= nums[left]) && (target > nums[middle] || target < nums[left]) {
            left = middle + 1;
        } else if target < nums[middle] || target > nums[right] {
            right = middle - 1;
        } else {
            left = middle + 1;
        }
    }
    -1
}
