// Solution for: https://leetcode.com/problems/add-two-numbers/

#include "linked_list/defs.hpp"

ListNode *addTwoNumbers(ListNode *l1, ListNode *l2)
{
    int carry = 0;
    ListNode *dummy = new ListNode(0);
    ListNode *head = dummy;

    while (l1 != nullptr && l2 != nullptr)
    {
        int val = l1->val + l2->val + carry;
        carry = val >= 10;
        if (carry)
        {
            val -= 10;
        }
        head->next = new ListNode(val);
        head = head->next;
        l1 = l1->next;
        l2 = l2->next;
    }

    l1 = l1 == nullptr ? l2 : l1;
    while (l1 != nullptr)
    {
        int val = l1->val + carry;
        carry = val >= 10;
        if (carry)
        {
            val -= 10;
        }
        head->next = new ListNode(val);
        head = head->next;
        l1 = l1->next;
    }

    if (carry)
    {
        head->next = new ListNode(1, nullptr);
    }

    head = dummy->next;
    delete dummy;
    return head;
}