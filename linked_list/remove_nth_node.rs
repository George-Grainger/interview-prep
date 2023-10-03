// Solution for: https://leetcode.com/problems/remove-nth-node-from-end-of-list

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut pre_head = Box::new(ListNode {
        val: -1,
        next: head,
    });
    let mut fast = pre_head.clone();
    for _ in 0..n {
        fast = fast.next.unwrap();
    }

    let mut slow = pre_head.as_mut();
    while fast.next.is_some() {
        fast = fast.next.unwrap();
        slow = slow.next.as_mut().unwrap();
    }
    slow.next = slow.next.as_mut().unwrap().next.take();
    pre_head.next
}
