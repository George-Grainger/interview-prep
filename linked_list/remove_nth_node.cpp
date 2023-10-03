// Solution for: https://leetcode.com/problems/remove-nth-node-from-end-of-list

#include "linked_list/defs.hpp"

ListNode *removeNthFromEnd(ListNode *head, int n)
{
    ListNode *runner = head;

    // Go one further than necessary to remove the node
    for (int i = 0; i < n; i++)
    {
        runner = runner->next;
    }

    if (runner == nullptr)
    {
        return head->next;
    }

    // Move to one in front of the node to remove
    ListNode *chaser = head;
    while (runner->next != nullptr)
    {
        runner = runner->next;
        chaser = chaser->next;
    }

    // Update the node - memory leak?
    // Maybe want to actually delete the node?
    chaser->next = chaser->next->next;
    return head;
}
