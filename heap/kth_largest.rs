/// Solution for: https://leetcode.com/problems/kth-largest-element-in-a-stream/

use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        nums.sort();
        let nums: Vec<Reverse<i32>> = nums.into_iter().rev().take(k as usize).map(|n| Reverse(n)).collect();

        KthLargest {
            k: k as usize,
            heap: BinaryHeap::from(nums)
        }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(Reverse(val));
        } else {
            let lowest = self.heap.peek().expect("Guaranteed k values").0;
            if lowest < val {
                self.heap.pop();
                self.heap.push(Reverse(val));
            }
        }
        self.heap.peek().expect("Guaranteed k values").0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */