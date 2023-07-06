// Solution for: https://leetcode.com/problems/valid-parentheses/description/

#[test]
fn test_valid_parenthesis() {
    assert(valid_parenthesis("()"));
    assert(valid_parenthesis("()[]{}"));
    assert(!valid_parenthesis("(]"));
}

fn valid_parenthesis(s: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }

    let mut p_stack: Vec<char> = Vec::with_capacity(s.len() / 2);
    for c in s.chars() {
        match c {
            '(' => p_stack.push(')'),
            '{' => p_stack.push('}'),
            '[' => p_stack.push(']'),
            ')' | '}' | ']' => {
                if Some(c) != p_stack.pop() {
                    return false;
                }
            }
            _ => panic!("Invalid character {}", c),
        };
    }
    p_stack.is_empty()
}
