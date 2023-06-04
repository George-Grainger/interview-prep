// Solution for: https://leetcode.com/problems/product-of-array-except-self/

#[test]
fn test_product_except_self() {
    assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    assert_eq!(
        product_except_self(vec![-1, 1, 0, -3, 3]),
        vec![0, 0, 9, 0, 0]
    );
}

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![1; nums.len()];

    nums.iter().enumerate().rev().fold(1, |acc, (i, &num)| {
        result[i] = acc;
        acc * num
    });

    nums.iter().enumerate().fold(1, |acc, (i, &num)| {
        result[i] *= acc;
        acc * num
    });

    result
}
