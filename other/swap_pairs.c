/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode* swapPairs(struct ListNode* head) {
    if(!head || !head->next) return head;

    struct ListNode* dummy = new ListNode();
    struct ListNode* prev = dummy;
    struct ListNode* curr = head;
    
    while(curr && curr->next){
        prev->next = curr->next;
        curr->next = prev->next->next;
        prev->next->next = curr;
        
        prev = curr;
        curr = curr->next;
    }
    
    return dummy->next;
}