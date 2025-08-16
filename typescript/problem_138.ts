// Definition for _Node.
class _Node {
    val: number
    next: _Node | null
    random: _Node | null

    constructor(val?: number, next?: _Node, random?: _Node) {
        this.val = (val === undefined ? 0 : val)
        this.next = (next === undefined ? null : next)
        this.random = (random === undefined ? null : random)
    }
}

function copyRandomList(head: _Node | null): _Node | null {
    if (!head) return head;

    const oldNodeToIndex = new Map<_Node, number>();
    const newIndexToNode = new Map<number, _Node>();

    const newHead: _Node | null = new _Node(head.val);

    let newCurr: _Node | null = newHead;
    let curr: _Node | null = head;
    let idx = 0;
    while (curr && newCurr) {
        if (curr.next) {
            newCurr.next = new _Node(curr.next.val);
        }

        oldNodeToIndex.set(curr, idx);
        newIndexToNode.set(idx, newCurr);

        newCurr = newCurr.next;
        curr = curr.next;
        idx++;
    }

    curr = head;
    idx = 0;
    while (curr) {
        if (curr.random) {
            const index = oldNodeToIndex.get(curr.random)!;
            newIndexToNode.get(idx)!.random = newIndexToNode.get(index)!;
        }
        idx++;
        curr = curr.next;
    }

    return newHead;
};