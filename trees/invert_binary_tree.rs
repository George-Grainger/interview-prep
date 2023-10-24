// Solution for: https://leetcode.com/problems/invert-binary-tree/

use std::rc::Rc;
use std::cell::RefCell;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_some() {
        let mut node = root.as_mut().unwrap().borrow_mut();
        let left = node.left.take();
        let right = node.right.take();
        node.left = invert_tree(right);
        node.right = invert_tree(left);
    }
    root
}