// Solution to: https://leetcode.com/problems/permutation-in-string/

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
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();

    let mut s1_counts: [usize; 26] = [0; 26];
    let mut s2_counts: [usize; 26] = [0; 26];

    for byte_char in s1 {
        s1_counts[(byte_char - b'a') as usize] += 1;
    }

    let mut left = 0;
    for right in 0..s2.len() {
        s2_counts[(s2[right] - b'a') as usize] += 1;
        if right - left == s1.len() {
            s2_counts[(s2[left] - b'a') as usize] -= 1;
            left += 1;
        }
        if s2_counts.eq(&s1_counts) {
            return true;
        }
    }

    false
}
