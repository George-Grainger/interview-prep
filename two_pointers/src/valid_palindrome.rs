fn is_palindrome(s: String) -> bool {
    let s = s
        .trim()
        .to_lowercase()
        .replace(|c| !char::is_alphanumeric(c), "");

    for (i, j) in s.chars().zip(s.chars().rev()) {
        if (i != j) {
            return false;
        }
    }

    true
}
