# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution(object):
    def sumRootToLeaf(self, root):
        def dfs(node, current_sum):
            if not node:
                return 0
            
            # Shift previous sum left by 1 and append current node's bit value
            current_sum = (current_sum << 1) | node.val
            
            # If it's a leaf node, return the completed binary number value
            if not node.left and not node.right:
                return current_sum
            
            # Otherwise, continue down both branches
            return dfs(node.left, current_sum) + dfs(node.right, current_sum)
            
        return dfs(root, 0)
