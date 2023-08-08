// Solution to: https://leetcode.com/problems/permutation-in-string/

use std::collections::HashMap;

#[test]
fn test_check_inclusion() {
    let s1 = String::from("ab");
    let s2 = String::from("eidbaooo");
    assert!(check_inclusion(s1, s2));

    let s1 = String::from("ab");
    let s2 = String::from("eidboaoo");
    assert!(!check_inclusion(s1, s2));

    let s1 = String::from("adc");
    let s2 = String::from("dcda");
    assert!(check_inclusion(s1, s2));

    let s1 = String::from("a");
    let s2 = String::from("ab");
    assert!(check_inclusion(s1, s2));
}

fn check_inclusion(s1: String, s2: String) -> bool {
    let mut s1_counts: [usize; 26] = [0; 26];

    for c in s1.as_bytes() {
        let idx = (c - b'a') as usize;
        s1_counts[idx] += 1;
    }

    for win in s2.as_bytes().windows(s1.len()) {
        let mut letters = s1_counts.clone();
        let mut is_match = true;
        for c in win {
            let idx = (c - b'a') as usize;
            if letters[idx] == 0 {
                is_match = false;
                break;
            } else {
                letters[idx] -= 1;
            }
        }
        if is_match {
            return true;
        }
    }

    false
}
