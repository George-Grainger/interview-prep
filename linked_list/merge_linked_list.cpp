// Solution for: https://leetcode.com/problems/merge-two-sorted-lists/

#include "linked_list/defs.hpp"

ListNode *mergeTwoLists(ListNode *list1, ListNode *list2)
{
    ListNode *combined_dummy = new ListNode(0);
    ListNode *combined_tail = combined_dummy;

    while (list1 != nullptr && list2 != nullptr)
    {
        if (list1->val < list2->val)
        {
            combined_tail->next = list1;
            list1 = list1->next;
        }
        else
        {
            combined_tail->next = list2;
            list2 = list2->next;
        }
        combined_tail = combined_tail->next;
    }

    if (list1 == nullptr)
    {
        combined_tail->next = list2;
    }
    else if (list2 == nullptr)
    {
        combined_tail->next = list1;
    }

    return combined_dummy->next;
}
