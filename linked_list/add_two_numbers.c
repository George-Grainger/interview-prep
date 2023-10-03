// Solution for: https://leetcode.com/problems/add-two-numbers/

#include "linked_list/defs.h"

typedef struct ListNode ListNode;

struct ListNode *addTwoNumbers(struct ListNode *l1, struct ListNode *l2)
{
    int carry = 0;
    ListNode *dummy = malloc(sizeof(ListNode));
    assert(dummy != NULL);
    ListNode *head = dummy;

    while (l1 != NULL && l2 != NULL)
    {
        int next_val = l1->val + l2->val + carry;
        carry = next_val >= 10;
        if (carry)
        {
            next_val -= 10;
        }
        head->next = malloc(sizeof(ListNode));
        head->next->val = next_val;

        l1 = l1->next;
        l2 = l2->next;
        head = head->next;
    }

    l1 = l1 == NULL ? l2 : l1;
    while (l1 != NULL)
    {
        int next_val = l1->val + carry;
        carry = next_val >= 10;
        if (carry)
        {
            next_val -= 10;
        }
        head->next = malloc(sizeof(ListNode));
        head->next->val = next_val;
        l1 = l1->next;
        head = head->next;
    }

    if (carry)
    {
        head->next = malloc(sizeof(ListNode));
        head->next->val = 1;
        head = head->next;
    }
    head->next = NULL;

    head = dummy->next;
    free(dummy);
    return head;
}