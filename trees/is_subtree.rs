// Solution for: https://leetcode.com/problems/subtree-of-another-tree/

fn is_same_tree(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() || q.is_none() {
        return p.is_none() && q.is_none();
    }

    let p_node = p.as_ref().unwrap().borrow();
    let q_node = q.as_ref().unwrap().borrow();

    p_node.val == q_node.val && 
    is_same_tree(p_node.left.as_ref(), q_node.left.as_ref()) && 
    is_same_tree(p_node.right.as_ref(), q_node.right.as_ref())
}

fn internal_subtree(root: Option<&Rc<RefCell<TreeNode>>>, sub_root: Option<&Rc<RefCell<TreeNode>>>) -> bool{
    if root.is_none() {
        return false;
    }

    if is_same_tree(root, sub_root) {
        return true;
    }

    let mut node = root.as_ref().unwrap().borrow();
    internal_subtree(node.left.as_ref(), sub_root) ||
    internal_subtree(node.right.as_ref(), sub_root) 
}

fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    sub_root.is_none() || internal_subtree(root.as_ref(), sub_root.as_ref())
}