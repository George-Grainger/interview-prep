use std::collections::HashSet;

impl Solution {
    fn _ss(nums: &[i32], seen: &mut HashSet<i32>, res: &mut Vec<Vec<i32>>) {
        if nums.len() == 1 {
            let initial_use = seen.insert(nums[0]);
            for mut subset in res.clone() {
                subset.push(nums[0]);
                if initial_use || !res.contains(&subset) {
                    res.push(subset)
                }
            }
        } else {
            Self::_ss(&nums[0..1], seen, res);
            Self::_ss(&nums[1..], seen, res);
        }
    }

    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = vec![vec![]];
        let mut seen = HashSet::new();
        Self::_ss(&nums[..], &mut seen, &mut res);
        res
    }
}
