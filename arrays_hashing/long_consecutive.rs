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

    nums.iter()
        .filter(|&n| !nums.contains(&(n - 1)))
        .map(|&n| (n..).take_while(|next| nums.contains(next)).count())
        .max()
        .unwrap_or_default() as i32
}
