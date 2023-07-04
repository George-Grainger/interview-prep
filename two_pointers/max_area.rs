use std::cmp;

#[test]
fn test_max_area() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(max_area(height), 49);

    let height = vec![1, 1];
    assert_eq!(max_area(height), 1);
}

#[inline(always)]
fn get_area(left: usize, right: usize, height: &Vec<i32>) -> i32 {
    let w = (right - left) as i32;
    let h = cmp::min(height[left], height[right]);
    w * h
}

fn max_area(height: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut end = height.len() - 1;
    let mut area = get_area(start, end, &height);

    while start < end {
        if height[start] < height[end] {
            start += 1;
        } else {
            end -= 1;
        }
        area = cmp::max(area, get_area(start, end, &height));
    }
}
