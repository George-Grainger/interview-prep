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
    let mut valid = true;
    for c in s.chars() {
        valid &= match c {
            '(' | '{' | '[' => {
                p_stack.push(c);
                true
            }
            ')' | '}' | ']' => match p_stack.pop() {
                Some('(') => c == ')',
                Some('{') => c == '}',
                Some('[') => c == ']',
                _ => false,
            },
            _ => panic!("Invalid character {}", c),
        };
    }
    valid && p_stack.is_empty()
}
