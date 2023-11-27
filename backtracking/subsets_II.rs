/// Solution for: https://leetcode.com/problems/subsets-ii/

fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    nums.sort();
    backtrack(&nums, 0, &mut Vec::new(), &mut result);
    result
}

fn backtrack(nums: &[i32], start: usize, subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    result.push(subset.clone());

    for i in start..nums.len() {
        if i > start && nums[i] == nums[i - 1] {
            continue;
        }

        subset.push(nums[i]);
        backtrack(nums, i + 1, subset, result);
        subset.pop();
    }
}
