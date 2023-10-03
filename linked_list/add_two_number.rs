fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut head = &mut dummy;

    let mut carry = false;

    while l1.is_some() || l2.is_some() || carry {
        let mut total = carry as i32;

        if let Some(node) = l1 {
            total += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            total += node.val;
            l2 = node.next;
        }

        carry = total >= 10;
        if (carry) {
            total -= 10;
        }

        head.next = Some(Box::new(ListNode::new(total)));
        head = head.next.as_mut().unwrap();
    }

    dummy.next
}
