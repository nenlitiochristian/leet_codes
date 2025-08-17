from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        if subRoot is None:
            return True
        if root is None:
            if subRoot is not None:
                return False
            return True

        if self.check_same(root, subRoot):
            return True
        
        return self.isSubtree(root.left, subRoot) or self.isSubtree(root.right, subRoot)

    def check_same(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]):
        if subRoot is None or root is None:
            if subRoot is not None:
                return False
            if root is not None:
                return False
            return True
        if root.val != subRoot.val:
            return False

        return self.check_same(root.left, subRoot.left) and self.check_same(
            root.right, subRoot.right
        )