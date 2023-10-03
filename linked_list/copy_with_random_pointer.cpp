// Solution for https://leetcode.com/problems/copy-list-with-random-pointer/

#include "linked_list/defs.hpp"

Node *copyRandomList(Node *head)
{
    if (head == nullptr)
    {
        return nullptr;
    }

    Node *current = head;
    while (current != nullptr)
    {
        Node *copy = new Node(current->val);
        copy->next = current->next;
        copy->random = nullptr;
        current->next = copy;
        current = copy->next;
    }

    current = head;
    while (current != nullptr)
    {
        if (current->random != nullptr)
        {
            Node *copy = current->next;
            copy->random = current->random->next;
        }
        current = current->next->next;
    }

    current = head;
    Node *new_head = current->next;
    while (current != nullptr)
    {
        Node *copy = current->next;
        current->next = copy->next;
        if (copy->next != nullptr)
        {
            copy->next = copy->next->next;
        }
        current = current->next;
    }

    return new_head;
}