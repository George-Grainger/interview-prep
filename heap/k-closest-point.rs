/// Solution for: https://leetcode.com/problems/k-closest-points-to-origin/

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        points.sort_unstable_by_key(|point| point.iter().map(|p| p * p).sum::<i32>());
        points[..k as usize].to_vec()
    }
}