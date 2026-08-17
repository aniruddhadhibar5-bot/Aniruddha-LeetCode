class Solution:
    def maxPathSum(self, root):
        self.max_sum = float('-inf')

        def dfs(node):
            if not node:
                return 0
            # Compute max path sum from left and right subtrees
            left = max(dfs(node.left), 0)
            right = max(dfs(node.right), 0)
            # Update global maximum if path through this node is better
            self.max_sum = max(self.max_sum, node.val + left + right)
            # Return max path sum extending upward
            return node.val + max(left, right)

        dfs(root)
        return self.max_sum
