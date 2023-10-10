// Solution for: https://leetcode.com/problems/merge-k-sorted-lists/submissions/

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut combined_dummy = ListNode::new(0);
    let mut combined_tail = &mut combined_dummy;
    let mut listnode_heap: BinaryHeap<Reverse<Box<ListNode>>> =
        lists.into_iter().flatten().map(|n| Reverse(n)).collect();

    while let Some(Reverse(min_node)) = listnode_heap.pop() {
        combined_tail.next = Some(min_node);
        combined_tail = combined_tail.next.as_mut().unwrap();

        if let Some(to_add) = combined_tail.next.take() {
            listnode_heap.push(Reverse(to_add));
        }
    }

    combined_dummy.next
}
