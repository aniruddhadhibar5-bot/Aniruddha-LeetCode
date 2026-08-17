from collections import defaultdict

class Solution:
    def findFrequentTreeSum(self, root):
        if not root:
            return []

        freq = defaultdict(int)

        def dfs(node):
            if not node:
                return 0
            s = node.val + dfs(node.left) + dfs(node.right)
            freq[s] += 1
            return s

        dfs(root)
        max_freq = max(freq.values())
        return [s for s, f in freq.items() if f == max_freq]
