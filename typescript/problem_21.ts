// Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.next = (next === undefined ? null : next)
    }
}

function mergeTwoLists(list1: ListNode | null, list2: ListNode | null): ListNode | null {
    if (!list1) return list2;
    if (!list2) return list1;

    let head: ListNode | null = null;
    if (list1.val < list2.val) {
        head = list1;
        list1 = list1.next;
    }
    else {
        head = list2;
        list2 = list2.next;
    }

    let curr = head;
    while (list1 && list2) {
        if (list1.val < list2.val) {
            curr.next = list1;
            list1 = list1.next;
        }
        else {
            curr.next = list2;
            list2 = list2.next;
        }
        curr = curr.next;
        curr.next = null;
    }
    if (list1) {
        curr.next = list1;
    }
    if (list2) {
        curr.next = list2;
    }
    return head;
};

function arrayToList(number: number[]) {
    const head = new ListNode(number[0], null);
    let curr = head;
    for (let i = 1; i < number.length; i++) {
        curr.next = new ListNode(number[i], null);
        curr = curr.next;
    }
    return head;
}

console.log(mergeTwoLists(arrayToList([1, 2, 4]), arrayToList([1, 3, 4])));