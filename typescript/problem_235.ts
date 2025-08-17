//Definition for a binary tree node.
class TreeNode {
    val: number
    left: TreeNode | null
    right: TreeNode | null
    constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.left = (left === undefined ? null : left)
        this.right = (right === undefined ? null : right)
    }
}


function lowestCommonAncestor(root: TreeNode | null, p: TreeNode | null, q: TreeNode | null): TreeNode | null {
    if (!root || !p || !q) return null;

    // this node is the one we're searching for
    // if p happens to be the ancestor of q,
    // we want to return p, that means we don't care if the children is p or q,
    // just return
    if (root.val === p.val || root.val === q.val) {
        return root;
    }

    // don't need to evaluate right
    if (p.val < root.val && q.val < root.val) {
        return lowestCommonAncestor(root.left, p, q);
    }
    if (p.val > root.val && q.val > root.val) {
        return lowestCommonAncestor(root.right, p, q);
    }

    // if we find p or q, we return it
    // if it's not either, return null
    // the way we know if this is the lowest ancestor is 
    // if this current node gets both p and q
    const left = lowestCommonAncestor(root.left, p, q);
    const right = lowestCommonAncestor(root.right, p, q);

    // we found both, this is ancestor!
    if ((left?.val === p.val && right?.val === q.val) ||
        (right?.val === p.val && left?.val === q.val)) {
        return root;
    }

    if (left) {
        return left;
    }
    if (right) {
        return right;
    }

    return null;
};