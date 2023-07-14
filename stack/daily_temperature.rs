// Solution to: https://leetcode.com/problems/daily-temperatures/

#[test]
fn test_daily_temperatures() {
    let tc = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];
    assert_eq!(daily_temperatures(tc), expected);

    let tc = vec![30, 40, 50, 60];
    let expected = vec![1, 1, 1, 0];
    assert_eq!(daily_temperatures(tc), expected);

    let tc = vec![30, 60, 90];
    let expected = vec![1, 1, 0];
    assert_eq!(daily_temperatures(tc), expected);

    let tc = vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70];
    let expected = vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0];
    assert_eq!(daily_temperatures(tc), expected);
}

fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = vec![0; temperatures.len()];

    for i in (0..temperatures.len() - 1).rev() {
        let mut j = i + 1;
        while temperatures[i] >= temperatures[j] && stack[j] != 0 {
            j += stack[j] as usize;
        }
        if temperatures[i] < temperatures[j] {
            stack[i] = (j - i) as i32;
        }
    }
    stack
}
