// Solution for: https://leetcode.com/problems/valid-anagram/

#[test]
fn test_is_anagram() {
    assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
    assert!(!is_anagram("rat".to_string(), "car".to_string()));
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut chars = [0; 26];

    s.bytes().zip(t.bytes()).for_each(|(u, v)| {
        chars[(u - b'a') as usize] += 1;
        chars[(v - b'a') as usize] -= 1;
    });

    chars.iter().all(|&c| c == 0)
}
