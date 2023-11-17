fn longest_palindrome(s: String) -> String {
    let n = s.len();
    let chars: Vec<char> = s.chars().collect();
    let mut dp = vec![vec![false; s.len()]; s.len()];
    let mut ans: (usize, usize) = (0, 0);

    for i in 0..n {
        dp[i][i] = true;
    }

    for (i, s) in s.as_bytes().windows(2).enumerate() {
        if s[0] == s[1] {
            dp[i][i + 1] = true;
            ans = (i, 1);
        }
    }

    for diff in 2..n {
        for i in 0..(n - diff) {
            let j = i + diff;
            if chars[i] == chars[j] && dp[i + 1][j - 1] {
                dp[i][j] = true;
                ans = (i, diff)
            }
        }
    }

    s.chars().skip(ans.0).take(ans.1 + 1).collect()
}
