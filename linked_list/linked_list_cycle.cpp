// Solution to: https://leetcode.com/problems/linked-list-cycle/

#include "linked_list/defs.hpp"

bool hasCycle(ListNode *head)
{
    ListNode *current = head;

    while (current != nullptr)
    {
        ListNode *next = current->next;
        if (current == next)
        {
            return true;
        }
        current->next = current;
        current = next;
    }

    return false;
}