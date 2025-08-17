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
        if root is None and subRoot is not None:
            return False
        if root is None and subRoot is None:
            return True

        # find first
        search = self.find_subroot(root, subRoot.val)

        if search is None:
            return False

        return self.check_same(search, subRoot)

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

    def find_subroot(self, root: Optional[TreeNode], target: int):
        if root is None or root.val == target:
            return root

        left = self.find_subroot(root.left, target)
        if left is not None:
            return left
        return self.find_subroot(root.right, target)
