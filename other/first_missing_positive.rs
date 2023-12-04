/// Naive solution for: https://leetcode.com/problems/first-missing-positive/

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let nums: HashSet<i32> = HashSet::from_iter(nums.into_iter());
        
        let mut i = 1;
        while nums.contains(&i) {
            i += 1;
        }
        i
    }
}