/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
/**
 * Note: The returned array must be malloced, assume caller calls free().
 */

#include <stdio.h>
#include <stdlib.h>

typedef struct ListNode {
    int val;
    struct ListNode *next;
} Node;

int* nodesBetweenCriticalPoints(struct ListNode* head, int* returnSize) {
    *returnSize = 2;
    int *ans = (int*) malloc(2 * sizeof(int));
    ans[0] = -1;
    ans[1] = -1;
    int first = -1;
    int prev = -1;
    int cur = 0;
    while (head != NULL && head->next != NULL && head->next->next != NULL) {
        if (head->next->val > head->val && head->next->val > head->next->next->val || 
            head->next->val < head->val && head->next->val < head->next->next->val) {
            if (first == -1) {
                first = prev = cur;
            } else {
                ans[1] = cur - first;
                if (ans[0] == -1 || cur - prev < ans[0]) {
                    ans[0] = cur - prev;
                }
                prev = cur;
            }
        }
        cur += 1;
        head = head->next;
    }
    return ans;
}

void print_list(Node *head) {
    int i = 0;
    while (head != NULL) {
        if (i > 0) {
            printf("->");
        }
        printf("%d", head->val);
        i += 1;
        head = head->next;
    }
    printf("\n");
}

int main(void) {
    int arr[] = {5,3,1,2,5,1,2};
    // Node *head = NULL;
    // print_list(head);
    // Node n = {1, head};
    // head = &n;

    // Node n1 = {2, head};
    // head = &n1;
    // print_list(head);
    int size = sizeof(arr) / sizeof(int);
    
    Node *head = NULL;
    for (int i = size - 1; i >= 0; i--) {
        Node *n = (Node*)malloc(sizeof(Node));
        n->val = arr[i];
        n->next = head;
        head = n;
    }
    print_list(head);
    int returnSize;
    int* ans = nodesBetweenCriticalPoints(head, &returnSize);
    printf("[%d, %d]\n", ans[0], ans[1]);
    free(ans);
}