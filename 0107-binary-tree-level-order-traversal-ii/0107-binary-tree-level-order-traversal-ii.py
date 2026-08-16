from collections import deque

class Solution:
    def levelOrderBottom(self, root):
        if not root:
            return []

        res = []
        queue = deque([root])

        while queue:
            level = []
            for _ in range(len(queue)):
                node = queue.popleft()
                level.append(node.val)
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)
            res.append(level)

        return res[::-1]  # reverse to get bottom-up order
