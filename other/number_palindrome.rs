/// Solution for: https://leetcode.com/problems/palindrome-number/

fn is_palindrome(x: i32) -> bool {
    let mut num = x;
    let mut reversed = 0;

    while num > 0 {
        reversed = 10 * reversed + num % 10;
        num /= 10;
    }

    x == reversed
}
