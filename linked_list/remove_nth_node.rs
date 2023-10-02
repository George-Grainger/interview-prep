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

// Similar C++
// ListNode* removeNthFromEnd(ListNode* head, int n) {
//     ListNode* runner = head;

//     // Go one further than necessary to remove the node
//     for(int i = 0; i < n; i++){
//         runner = runner->next;
//     }

//     if (runner == nullptr) {
//         return head->next;
//     }

//     // Move to one in front of the node to remove
//     ListNode* chaser = head;
//     while(runner->next != nullptr) {
//         runner = runner->next;
//         chaser = chaser->next;
//     }

//     // Update the node - memory leak?
//     // Maybe want to actually delete the node?
//     chaser->next = chaser->next->next;
//     return head;
// }
