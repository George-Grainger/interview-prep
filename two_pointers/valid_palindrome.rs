// Solution for: https://leetcode.com/problems/valid-palindrome/

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome(String::from(
        "A man, a plan, a canal: Panama"
    )));
    assert!(!is_palindrome(String::from("race a car")));
    assert!(is_palindrome(String::from(" ")));
}

fn is_palindrome(s: String) -> bool {
    let s = s.replace(|c| !char::is_alphanumeric(c), "").to_lowercase();
    s.chars().zip(s.chars().rev()).all(|(i, j)| i == j)
}
