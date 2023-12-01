/// Solution for: https://leetcode.com/problems/search-insert-position/

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while right > left {
            let pivot = (left + right) / 2;

            if nums[pivot] < target {
                left = pivot + 1;
            } else {
                right = pivot;
            }
        }

        left as i32
    }
}
