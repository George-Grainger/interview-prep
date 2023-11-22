/// Solution for: https://leetcode.com/problems/palindrome-number/

fn is_palindrome(x: i32) -> bool {
    let x = x.to_string();
    let mut bytes = x.bytes();

    loop {
        let front = bytes.next();
        let back = bytes.next_back();

        if front.is_none() || back.is_none() {
            return true;
        }

        if front != back {
            return false;
        }
    }
}
