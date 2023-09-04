// Solution for: https://leetcode.com/problems/longest-repeating-character-replacement/

#[test]
fn test_character_replacement() {
    let s = String::from("ABAB");
    let k = 2;
    assert_eq!(character_replacement(s, k), 4);

    let s = String::from("AABABBA");
    let k = 1;
    assert_eq!(character_replacement(s, k), 4);

    let s = String::from("AAAA");
    let k = 2;
    assert_eq!(character_replacement(s, k), 4);

    let s = String::from("ABBB");
    let k = 2;
    assert_eq!(character_replacement(s, k), 4);

    let s = String::from("ABAA");
    let k = 0;
    assert_eq!(character_replacement(s, k), 2);

    let s = String::from("BAAAB");
    let k = 2;
    assert_eq!(character_replacement(s, k), 5);
}

fn character_replacement(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let k = k as usize;

    let mut counts = [0; 26];
    let mut max_count: usize = 0;

    let mut left: usize = 0;
    for (right, b) in s.iter().map(|b| (b - b'A') as usize).enumerate() {
        counts[b] += 1;
        max_count = usize::max(max_count, counts[b]);

        let win_size = right - left + 1;
        if win_size - max_count > k {
            counts[(s[left] - b'A') as usize] -= 1;
            left += 1;
        }
    }

    (s.len() - left) as i32
}
