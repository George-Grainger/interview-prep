use std::collections::HashMap;

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
    let mut max_consecutive = 0;
    let mut output: HashMap<i32, i32> = HashMap::with_capacity(nums.len() / 4);

    for num in nums {
        if !output.contains_key(&num) {
            output.insert(num, num);

            let below = *output.get(&(num - 1)).unwrap_or(&num);
            let above = *output.get(&(num + 1)).unwrap_or(&num);

            output.insert(below, above);
            output.insert(above, below);

            max_consecutive = max_consecutive.max((above - below) + 1);
        }
    }
    max_consecutive
}
