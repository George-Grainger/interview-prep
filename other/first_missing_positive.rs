/// Naive solution for: https://leetcode.com/problems/first-missing-positive/

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut lowest = i32::MAX;
        let mut highest = 0;
        let mut sum = 0;

        for num in nums {
            if num > 0 {
                lowest = lowest.min(num);
                highest = highest.max(num);
                sum += num;
            }
        }

        // If 1 hasn't been seen return this
        if lowest != 1 {
            return 1;
        }

        // Calculate difference between expected sum and seen sum
        let expected_sum = (highest) * (highest + 1) / 2;
        let diff = expected_sum - sum;

        // If there's no difference the next value is the first missing
        // Otherwise the difference is the missing number
        if diff == 0 {
            highest + 1
        } else {
            diff
        }
    }
}