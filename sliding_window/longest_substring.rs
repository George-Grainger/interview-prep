// Solution for: https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::cmp;
use std::collections::HashMap;

const N_CHARS: usize = 52;

#[test]
fn test_longest_substr() {
    let s = String::from("abcabcbb");
    assert_eq!(longest_substr(s), 3);

    let s = String::from("bbbbb");
    assert_eq!(longest_substr(s), 1);

    let s = String::from("pwwkew");
    assert_eq!(longest_substr(s), 3);

    let s = String::from("abcdefgh");
    assert_eq!(longest_substr(s), 8);

    let s = String::from("cdd");
    assert_eq!(longest_substr(s), 2);

    let s = String::from("aaaaabcde");
    assert_eq!(longest_substr(s), 5);

    let s = String::from("abba");
    assert_eq!(longest_substr(s), 2);

    let s = String::from("tmmzuxt");
    assert_eq!(longest_substr(s), 5);
}

fn longest_substr(s: String) -> i32 {
    // Number of letterlongest_substr(s), symbols etc?
    let mut map: HashMap<char, isize> = HashMap::with_capacity(N_CHARS);
    let mut max_substr = 0;
    let mut lp = -1;

    for (idx, c) in s.chars().into_iter().enumerate() {
        let i_idx = idx as isize;
        if let Some(prev_idx) = map.insert(c, i_idx) {
            lp = cmp::max(lp, prev_idx);
        }
        max_substr = cmp::max(i_idx - lp, max_substr);
    }

    max_substr as i32
}
