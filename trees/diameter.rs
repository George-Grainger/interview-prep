// Solution for: https://leetcode.com/problems/diameter-of-binary-tree/

use std::rc::Rc;
use std::cell::RefCell;

fn diameter_internal(mut root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if root.is_none() {
        return (0, 0);
    }

    let mut node = root.as_mut().unwrap().borrow_mut();

    // Calculate the heights of the internal nodes
    let (l_breadth, l_height) = diameter_internal(node.left.take());
    let (r_breadth, r_height) = diameter_internal(node.right.take());

    // Get the height of the tree
    let height = 1 + l_height.max(r_height);

    // Calculate longest distance from left node to right node
    let breadth = l_height + r_height;

    // Check if this is longer than the internal diameters of both left and right
    let max_breadth = breadth.max(l_breadth).max(r_breadth);

    // Return the diameter and height 
    (max_breadth, height)
}

fn diameter_of_binary_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    diameter_internal(root).0
}