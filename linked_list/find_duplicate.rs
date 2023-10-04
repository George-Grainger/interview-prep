// Solution for: https://leetcode.com/problems/find-the-duplicate-number/

fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut slow = nums[0] as usize;
    let mut fast = nums[slow] as usize;

    while fast != slow {
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
    }

    slow = 0;
    while fast != slow {
        slow = nums[slow] as usize;
        fast = nums[fast] as usize;
    }

    slow as i32
}
