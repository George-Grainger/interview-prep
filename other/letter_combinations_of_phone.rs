/// Solution for https://leetcode.com/problems/letter-combinations-of-a-phone-number/

fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return Vec::new();
    }

    let mut combinations: Vec<Vec<char>> = vec![vec![]];

    for d in digits.chars() {
        let to_append = match d {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _ => unreachable!("Digits should be between 2 and 9"),
        };

        let n = combinations.len();
        let m = to_append.len();
        combinations = combinations.into_iter().cycle().take(n * m).collect();

        for i in 0..n * m {
            combinations[i].push(to_append[i / n]);
        }
    }

    combinations.iter().map(|c| c.iter().collect()).collect()
}
