// Solution for https://leetcode.com/problems/copy-list-with-random-pointer/

#include "linked_list/defs.h"

typedef struct Node Node;

struct Node *copyRandomList(struct Node *head)
{
    if (head == NULL)
    {
        return NULL;
    }

    Node *current = head;
    while (current != NULL)
    {
        Node *copy = malloc(sizeof(Node));
        assert(copy != NULL);
        copy->val = current->val;
        copy->next = current->next;
        current->next = copy;
        current = copy->next;
    }

    current = head;
    while (current != NULL)
    {
        Node *copy = current->next;
        if (current->random)
        {
            copy->random = current->random->next;
        }
        else
        {
            copy->random = NULL;
        }
        current = current->next->next;
    }

    current = head;
    Node *copy_head = head->next;
    while (current != NULL)
    {
        Node *copy = current->next;
        current->next = copy->next;
        if (copy->next != NULL)
        {
            copy->next = copy->next->next;
        }
        current = current->next;
    }
    return copy_head;
}