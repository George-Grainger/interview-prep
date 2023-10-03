// Solution for: https://leetcode.com/problems/reorder-list/submissions/

#include "linked_list/defs.hpp"

void reorderList(ListNode *head)
{

    // Calculate the number of nodes in the list
    int nodes = 0;
    ListNode *curr = head;
    while (curr != nullptr)
    {
        nodes++;
        curr = curr->next;
    }

    // Move to half way
    curr = head;
    for (int i = 0; i < nodes / 2; i++)
    {
        curr = curr->next;
    }

    // Reverse the second half
    ListNode *prev = nullptr;
    while (curr != nullptr)
    {
        ListNode *next = curr->next;
        curr->next = prev;
        prev = curr;
        curr = next;
    }

    // Use two pointers to alternate the list
    ListNode *front = head;
    ListNode *back = prev;
    for (int i = 0; i < (nodes - 1) / 2; i++)
    {
        ListNode *next_front = front->next;
        ListNode *next_back = back->next;
        front->next = back;
        back->next = next_front;
        front = next_front;
        back = next_back;
    }
}