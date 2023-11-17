impl Solution {
    fn expand(i: usize, j: usize, chars: &[u8]) -> usize {
        let mut left = i;
        let mut right = j;

        while left as isize >= 0 && right < chars.len() && chars[left] == chars[right] {
            left -= 1;
            right += 1;
        }

        right - left - 1
    }

    pub fn longest_palindrome(s: String) -> String {
        let mut ans = [0, 0];

        for i in 0..s.len() {
            let odd_len = Self::expand(i, i, &s.as_bytes());
            if (odd_len > ans[1]) {
                let dist = odd_len / 2;
                ans = [i - dist, odd_len];
            }

            let even_len = Self::expand(i, i + 1, &s.as_bytes());
            if (even_len > ans[1]) {
                let dist = (even_len / 2) - 1;
                ans = [i - dist, even_len]
            }
        }

        s.chars().skip(ans[0]).take(ans[1]).collect()
    }
}
