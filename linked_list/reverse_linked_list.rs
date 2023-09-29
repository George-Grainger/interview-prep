// Solution for: https://leetcode.com/problems/reverse-linked-list/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut next = head;
    while next.is_some() {
        let mut cur = next.unwrap(); // Box<ListNode>
        next = cur.next.take(); // next = next.next ; become next
        cur.next = prev;
        prev = Some(cur);
    }
    prev
}
