class Solution:
    def pathSum(self, root, targetSum):
        res = []

        def dfs(node, path, total):
            if not node:
                return
            path.append(node.val)
            total += node.val

            # If it's a leaf and sums match, record the path
            if not node.left and not node.right and total == targetSum:
                res.append(list(path))
            else:
                dfs(node.left, path, total)
                dfs(node.right, path, total)

            path.pop()  # backtrack

        dfs(root, [], 0)
        return res
