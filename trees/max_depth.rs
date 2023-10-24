// Solution for: https://leetcode.com/problems/maximum-depth-of-binary-tree/

use std::rc::Rc;
use std::cell::RefCell;

fn max_depth(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut node = root.as_mut().unwrap().borrow_mut();
    let l_depth = max_depth(node.left.take());
    let r_depth = max_depth(node.right.take());

    1 + l_depth.max(r_depth)
}
