/// Solution for: https://leetcode.com/problems/permutations/

fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() <= 1 {
        return vec![nums];
    } else {
        let val = nums
            .pop()
            .expect("Nums will always contain at least one number");
        let vec = permute(nums);
        let mut ret = Vec::new();
        for v in vec {
            for i in 0..=v.len() {
                let mut t = v.clone();
                t.insert(i, val);
                ret.push(t);
            }
        }
        return ret;
    }
}
