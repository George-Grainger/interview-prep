// Solution for: https://leetcode.com/problems/balanced-binary-tree

fn max_depth(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut node = root.as_ref().unwrap().borrow();
    let l_depth = max_depth(node.left.as_ref());
    let r_depth = max_depth(node.right.as_ref());

    1 + l_depth.max(r_depth)
}

fn balanced_internal(root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }

    let node = root.as_ref().unwrap().borrow();

    let left = node.left.as_ref();
    let right = node.right.as_ref();
    let height_diff = max_depth(left) - max_depth(right);

    height_diff.abs() <= 1 && balanced_internal(left) && balanced_internal(right)
}

fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    balanced_internal(root.as_ref())
}
