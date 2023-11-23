// Solution for: https://leetcode.com/problems/reverse-nodes-in-k-group/

ListNode *reverseKGroup(ListNode *head, int k)
{
    ListNode *dummy = new ListNode(0, head);
    ListNode *prev = dummy;

    int j = 0;
    while (prev != nullptr && prev->next != nullptr)
    {
        ListNode *curr = prev->next;
        ListNode *tmp = curr;

        // Find next pointer
        for (int i = 1; i < k; i++)
        {
            tmp = tmp->next;
            if (tmp == nullptr)
            {
                return dummy->next;
            }
        }

        // Update pointers
        ListNode *copy = curr;
        ListNode *tmp2 = copy->next;
        prev->next = tmp;
        curr->next = tmp->next;

        // Reverse pointers
        for (int i = 1; i < k; i++)
        {
            prev = curr;
            curr = tmp2;
            tmp2 = curr->next;
            curr->next = prev;
        }

        // Update next
        prev = copy;
    }

    return dummy->next;
}