// Solution for: https://leetcode.com/problems/generate-parentheses/

#[test]
fn test_generate_parenthesis() {
    let to_string_vec = |v: Vec<&str>| v.iter().map(|e| e.to_string()).collect::<Vec<String>>();

    let expected = to_string_vec(vec!["()"]);
    assert_eq!(generate_parenthesis(1), expected);

    let expected = to_string_vec(vec!["(())", "()()"]);
    assert_eq!(generate_parenthesis(2), expected);

    let expected = to_string_vec(vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    assert_eq!(generate_parenthesis(3), expected);

    let expected = to_string_vec(vec![
        "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
        "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
    ]);
    assert_eq!(generate_parenthesis(4), expected);
}

fn recurse(n: i32, open: i32, close: i32, curr_str: String, mut combinations: &mut Vec<String>) {
    if open == n && close == n {
        combinations.push(curr_str.to_string());
        return;
    }
    if open < n {
        let next_str = curr_str.clone() + "(";
        recurse(n, open + 1, close, next_str, &mut combinations);
    }
    if close < open {
        let next_str = curr_str.clone() + ")";
        recurse(n, open, close + 1, next_str, &mut combinations);
    }
}

fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut combinations: Vec<String> = Vec::with_capacity((n * n) as usize);
    recurse(n, 0, 0, String::new(), &mut combinations);
    return combinations;
}
