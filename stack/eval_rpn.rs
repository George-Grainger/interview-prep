// Solution for: https://leetcode.com/problems/evaluate-reverse-polish-notation/solutions/

#[test]
fn test_eval_rpn() {
    let vec_to_string = |v: Vec<&str>| v.iter().map(|e| e.to_string()).collect::<Vec<String>>();

    let in_vec = vec_to_string(vec!["2", "1", "+", "3", "*"]);
    assert_eq!(eval_rpn(in_vec), 9);

    let in_vec = vec_to_string(vec!["4", "13", "5", "/", "+"]);
    assert_eq!(eval_rpn(in_vec), 6);

    let in_vec = vec_to_string(vec![
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ]);
    assert_eq!(eval_rpn(in_vec), 22);
}

fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::with_capacity(tokens.len());

    for token in tokens {
        match token.parse::<i32>() {
            Ok(parsed) => stack.push(parsed),
            Err(_) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                stack.push(match token.as_str() {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    _ => panic!("Unsupported token"),
                })
            }
        };
    }

    stack.pop().unwrap_or_default()
}
