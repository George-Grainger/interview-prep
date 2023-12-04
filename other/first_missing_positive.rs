/// Naive solution for: https://leetcode.com/problems/first-missing-positive/

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut i = 0;

        // Swap numbers into their position in the array
        while i < nums.len() {
            if nums[i] > 0 && nums[i] <= n && nums[nums[i] as usize - 1] != nums[i] {
                let j = nums[i] as usize - 1;
                nums.swap(i, j);
            } else {
                i += 1;
            }
        }

        // Find the first that isn't correctly positioned
        for (i, &num) in nums.iter().enumerate() {
            let i_32 = i as i32;
            if num != i_32 + 1 {
                return i_32 + 1;
            }
        }

        // If they all are return the next positive number
        n + 1
    }
}
