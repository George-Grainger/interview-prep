/// Solution for: https://leetcode.com/problems/subsets/

fn _ss(nums: &[i32], res: &mut Vec<Vec<i32>>) {
    if nums.len() == 1 {
        for mut subset in res.clone() {
            subset.push(nums[0]);
            res.push(subset)
        }
    } else {
        _ss(&nums[0..1], res);
        _ss(&nums[1..], res);
    }
}

fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![vec![]];
    _ss(&nums[..], &mut res);
    res
}
