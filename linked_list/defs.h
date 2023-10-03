#ifndef DEFS
#define DEF
#include <stddef.h>

struct ListNode
{
    int val;
    struct ListNode *next;
};

struct Node
{
    int val;
    struct Node *next;
    struct Node *random;
};
#endif