#[test]
fn test_three_sum() {
    assert_eq!(
        three_sum(vec![-1, 0, 1, 2, -1, 4]),
        Vec::from([[-1, -1, 2], [-1, 0, 1]])
    );
    assert_eq!(three_sum(vec![0, 1, 1]), Vec::from([[]; 0]));
    assert_eq!(three_sum(vec![0, 0, 0, 0]), Vec::from([[0, 0, 0]]));
    assert_eq!(three_sum(vec![-1, 0, 1]), Vec::from([[-1, 0, 1]]));
}

fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut solutions = Vec::new();
    nums.sort();

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum > 0 {
                right -= 1;
            } else if sum < 0 {
                left += 1
            } else {
                solutions.push(vec![nums[i], nums[left], nums[right]]);
                left += 1;

                while nums[left] == nums[left - 1] && left < right {
                    left += 1;
                }
            }
        }
    }
    solutions
}
