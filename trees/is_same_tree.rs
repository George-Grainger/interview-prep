// Solution for: https://leetcode.com/problems/same-tree/

use std::rc::Rc;
use std::cell::RefCell;

fn is_same_tree(mut p: Option<Rc<RefCell<TreeNode>>>, mut q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() || q.is_none() {
        return p.is_none() && q.is_none();
    }

    let mut p_node = p.as_mut().unwrap().borrow_mut();
    let mut q_node = q.as_mut().unwrap().borrow_mut();

    p_node.val == q_node.val && 
    is_same_tree(p_node.left.take(), q_node.left.take()) && 
    is_same_tree(p_node.right.take(), q_node.right.take())
}