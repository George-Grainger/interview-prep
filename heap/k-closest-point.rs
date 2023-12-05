/// Solution for: https://leetcode.com/problems/k-closest-points-to-origin/

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;

        let mut distances = BinaryHeap::new();
        for point in points {
            let dist: i32 = point.iter().map(|p| p.pow(2)).sum();
            distances.push((Reverse(dist), point));
        }

        let mut closest = Vec::with_capacity(k);
        for i in 0..k {
            let point = distances.pop().expect("Should be at least k points");
            closest.push(point.1);
        }

        closest
    }
}