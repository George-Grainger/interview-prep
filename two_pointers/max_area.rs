use std::cmp;

#[test]
fn test_max_area() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(max_area(height), 49);

    let height = vec![1, 1];
    assert_eq!(max_area(height), 1);
}

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
        let li_area = get_area(start + 1, end, &height);
        if li_area > area {
            area = li_area;
            start += 1;
            continue;
        }

        let ri_area = get_area(start, end - 1, &height);
        if ri_area > area {
            area = ri_area;
            end -= 1;
            continue;
        }

        if height[start] < height[end] {
            start += 1;
        } else {
            end -= 1;
        }
    }

    area
}
