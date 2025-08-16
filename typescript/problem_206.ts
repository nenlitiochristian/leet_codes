// Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.next = (next === undefined ? null : next)
    }
}


function reverseList(head: ListNode | null): ListNode | null {
    if (!head) return null;
    let tail = new ListNode(head.val, null);
    let curr: ListNode | null = head.next;
    while (curr) {
        tail = new ListNode(curr.val, tail);
        curr = curr.next;
    }
    return tail;
};