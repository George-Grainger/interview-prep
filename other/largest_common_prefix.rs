/// Solution for: https://leetcode.com/problems/longest-common-prefix/
///
fn longest_common_prefix(strs: Vec<String>) -> String {
    let strs: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();

    let mut i = 0;
    let mut done = false;

    for &base in strs[0].iter() {
        for w in strs.iter().skip(1) {
            if i >= w.len() || base != w[i] {
                done = true;
                break;
            }
        }

        if done {
            break;
        }

        i += 1;
    }

    let prefix = Vec::from(&strs[0][0..i]);
    String::from_utf8(prefix).unwrap_or_default()
}
