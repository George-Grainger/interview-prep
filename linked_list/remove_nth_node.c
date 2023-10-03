// Solution for: https://leetcode.com/problems/remove-nth-node-from-end-of-list

#include "linked_list/defs.h"

struct ListNode *removeNthFromEnd(struct ListNode *head, int n)
{
    struct ListNode *fast = head;
    for (int i = 0; i < n; i++)
    {
        fast = fast->next;
    }

    if (fast == NULL)
    {
        return head->next;
    }

    struct ListNode *slow = head;
    while (fast->next != NULL)
    {
        slow = slow->next;
        fast = fast->next;
    }

    slow->next = slow->next->next;
    return head;
}