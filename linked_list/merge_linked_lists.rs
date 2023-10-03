// Solution for: https://leetcode.com/problems/merge-two-sorted-lists/

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

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut combined_dummy = ListNode::new(0);
        let mut combined_tail = &mut combined_dummy;

        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            if node1.val < node2.val {
                combined_tail.next = list1.take();
                combined_tail = combined_tail.next.as_mut().unwrap();
                list1 = combined_tail.next.take();
            } else {
                combined_tail.next = list2.take();
                combined_tail = combined_tail.next.as_mut().unwrap();
                list2 = combined_tail.next.take();
            }
        }
        combined_tail.next = list1.or(list2);

        combined_dummy.next
    }
}
