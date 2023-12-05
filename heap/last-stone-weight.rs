/// Solution for: https://leetcode.com/problems/last-stone-weight/

use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = BinaryHeap::from(stones);
    
        while stones.len() > 1 {
            let y = stones.pop().expect("Stones should have at least two elements");
            let x = stones.pop().expect("Stones should have at least two elements");
            
            if y != x {
                // Using max binary heap y >= x
                stones.push(y - x);
            }
        }

        stones.pop().unwrap_or(0)
    }
}