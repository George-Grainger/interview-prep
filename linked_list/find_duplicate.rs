fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut seen = vec![false; nums.len()];

    for num in nums {
        if (seen[num as usize] == true) {
            return num;
        } else {
            seen[num as usize] = true;
        }
    }

    0
}
