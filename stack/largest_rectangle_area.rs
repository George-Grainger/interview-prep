// Solution for: https://leetcode.com/problems/largest-rectangle-in-histogram/

use std::cmp;

#[test]
fn test_largest_rectanglge_area() {
    let output = largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]);
    assert_eq!(output, 10);

    let output = largest_rectangle_area(vec![2, 4]);
    assert_eq!(output, 4);

    let output = largest_rectangle_area(vec![2, 1, 5, 6, 3, 3]);
    assert_eq!(output, 12);

    let output = largest_rectangle_area(vec![1, 2, 2]);
    assert_eq!(output, 4);

    let output = largest_rectangle_area(vec![1, 2, 3, 4, 5]);
    assert_eq!(output, 9);

    let output = largest_rectangle_area(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
    assert_eq!(output, 6);
}

fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
    let mut area: i32 = 0;
    let mut stack: Vec<usize> = Vec::new();
    heights.push(0);
    heights.insert(0, 0);

    for (i, &height) in heights.iter().enumerate() {
        while !stack.is_empty() && heights[*stack.iter().last().unwrap()] > height {
            let j = stack.pop().unwrap();
            let width = (i - stack[stack.len() - 1] - 1) as i32;
            area = cmp::max(area, width * heights[j]);
        }
        stack.push(i);
    }
    area
}
