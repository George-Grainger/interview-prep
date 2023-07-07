// Solution for: https://leetcode.com/problems/min-stack/

use std::cmp;

#[test]
fn test_min_stack() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.get_min(), -3); // return -3
    min_stack.pop();
    assert_eq!(min_stack.top(), 0);    // return 0
    assert_eq!(min_stack.get_min(), -2); // return -2
}

struct StackElement {
    val: i32,
    min: i32,
}

struct MinStack {
    stack: Vec<StackElement>,
    min: i32,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min: i32::MAX,
        }
    }
    
    fn push(&mut self, val: i32) {
        let elem = StackElement {val: val, min: self.min};
        self.stack.push(elem);
        self.min = cmp::min(self.min, val);
    }
    
    fn pop(&mut self) {
        let value = self.stack.pop().unwrap();
        self.min = value.min;
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().val
    }
    
    fn get_min(&self) -> i32 {
        self.min
    }
}