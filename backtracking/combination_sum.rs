/// Solution for: https://leetcode.com/problems/combination-sum/

fn dfs(candidates: &[i32], target: i32, path: &mut Vec<i32>, solutions: &mut Vec<Vec<i32>>) {
    if target == 0 {
        solutions.push(path.to_vec());
    } else if target > 0 {
        for (i, &val) in candidates.iter().enumerate() {
            path.push(val);
            dfs(&candidates[i..], target - val, path, solutions);
            path.pop();
        }
    }
}

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut solutions = Vec::new();
    let mut path = Vec::new();

    dfs(&candidates, target, &mut path, &mut solutions);
    solutions
}
