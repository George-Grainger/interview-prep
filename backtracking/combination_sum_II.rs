/// Solution to: https://leetcode.com/problems/combination-sum-ii/

fn dfs(candidates: &[i32], target: i32, path: &mut Vec<i32>, solutions: &mut Vec<Vec<i32>>) {
    if target == 0 {
        solutions.push(path.to_vec());
    } else if target > 0 {
        for (i, &val) in candidates.iter().enumerate() {
            if i > 0 && candidates[i] == candidates[i - 1] {
                continue;
            }

            path.push(val);
            Self::dfs(&candidates[i + 1..], target - val, path, solutions);
            path.pop();
        }
    }
}

fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort();
    let mut solutions = Vec::new();
    let mut path = Vec::new();

    Self::dfs(&candidates, target, &mut path, &mut solutions);
    solutions
}
