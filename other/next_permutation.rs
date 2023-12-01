/// Solution for: https://leetcode.com/problems/next-permutation/

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut n = nums.len();
        let mut i = n - 1;

        // Find first decreasing element
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }

        // Find element to swap with
        if i > 0 {
            let mut j = n - 1;
            while j >= i && nums[j] <= nums[i - 1] {
                j -= 1;
            }
            nums.swap(i - 1, j);
        }

        // Reverse all elements after the swap
        nums[i..n].reverse();
    }
}
