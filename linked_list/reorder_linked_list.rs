// Solution for: https://leetcode.com/problems/reorder-list/submissions/

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

fn reorder_list(head: &mut Option<Box<ListNode>>) {
    if head.is_none() {
        return;
    }

    let mut queue = VecDeque::new();
    let mut curr = head.take();

    while let Some(mut node) = curr {
        curr = node.next.take();
        queue.push_back(node);
    }

    let mut front = queue.pop_front().unwrap();
    let mut curr = &mut front;

    for i in 0..queue.len() {
        let node = if i % 2 == 0 {
            queue.pop_back().unwrap()
        } else {
            queue.pop_front().unwrap()
        };

        curr.next = Some(node);
        curr = curr.next.as_mut().unwrap();
    }

    *head = Some(front);
}

// C++ solution
// void reorderList(ListNode* head) {

//     // Calculate the number of nodes in the list
//     int nodes = 0;
//     ListNode* curr = head;
//     while (curr != nullptr) {
//         nodes++;
//         curr = curr->next;
//     }

//     // Move to half way
//     curr = head;
//     for (int i = 0; i < nodes / 2; i++) {
//         curr = curr->next;
//     }

//     // Reverse the second half
//     ListNode* prev = nullptr;
//     while (curr != nullptr) {
//         ListNode* next = curr->next;
//         curr->next = prev;
//         prev = curr;
//         curr = next;
//     }

//     // Use two pointers to alternate the list
//     ListNode* front = head;
//     ListNode* back = prev;
//     for (int i = 0; i < (nodes - 1) / 2; i++) {
//         ListNode* next_front = front->next;
//         ListNode* next_back = back->next;
//         front->next = back;
//         back->next = next_front;
//         front = next_front;
//         back = next_back;
//     }
// }
