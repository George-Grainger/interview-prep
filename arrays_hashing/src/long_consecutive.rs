use std::collections::HashSet;

#[test]
fn test_longest_consecutive() {
    assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    assert_eq!(
        longest_consecutive(vec![-2, -3, -3, 7, -3, 0, 5, 0, -8, -4, -1, 2]),
        5
    );
}

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let nums: HashSet<i32> = nums.into_iter().collect();
    let mut max_sequence = 0;

    for &num in &nums {
        if nums.contains(&(num - 1)) {
            continue;
        }

        let mut current_num = num;
        let mut sequence = 1;
        // Remains O(n) since numbers can only be in one sequence
        while nums.contains(&(current_num + 1)) {
            sequence += 1;
            current_num += 1
        }

        max_sequence = max_sequence.max(sequence);
    }

    max_sequence
}
