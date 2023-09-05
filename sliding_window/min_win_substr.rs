// Solution for: https://leetcode.com/problems/minimum-window-substring/

use std::collections::HashMap;

#[test]
fn test_min_window() {
    let s = String::from("ADOBECODEBANC");
    let t = String::from("ABC");
    assert_eq!(min_window(s, t), String::from("BANC"));

    let s = String::from("a");
    let t = String::from("a");
    assert_eq!(min_window(s, t), String::from("a"));

    let s = String::from("a");
    let t = String::from("aa");
    assert_eq!(min_window(s, t), String::from(""));
}

fn min_window(s: String, t: String) -> String {
    let s = s.as_bytes();

    let mut map: HashMap<u8, i32> = HashMap::with_capacity(52);
    for &byte in t.as_bytes() {
        *map.entry(byte).or_insert(0) += 1;
    }

    let mut left = 0;
    let mut right = 0;
    let mut win = (0, s.len() + 1);
    while right < s.len() {
        while map.values().any(|&count| count > 0) && right < s.len() {
            if let Some(byte) = map.get_mut(&s[right]) {
                *byte -= 1;
            }
            right += 1;
        }
        while map.values().all(|&count| count <= 0) && left <= right {
            if let Some(byte) = map.get_mut(&s[left]) {
                *byte += 1;
            }
            left += 1;
        }

        if right - left < win.1 - win.0 {
            win = (left, right);
        }
    }

    if left == 0 {
        String::new()
    } else {
        let win = (win.0 - 1)..win.1;
        String::from_utf8(s[win].to_vec()).unwrap()
    }
}
