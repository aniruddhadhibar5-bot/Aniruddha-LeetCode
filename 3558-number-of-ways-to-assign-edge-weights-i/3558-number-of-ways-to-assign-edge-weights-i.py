class Solution:
    def assignEdgeWeights(self, edges):
        from collections import defaultdict, deque

        n = len(edges) + 1  # number of nodes in a tree = edges + 1

        # Build adjacency list
        graph = defaultdict(list)
        for u, v in edges:
            graph[u].append(v)
            graph[v].append(u)

        # BFS from root (node 1) to find maximum depth
        depth = {1: 0}
        q = deque([1])
        while q:
            node = q.popleft()
            for nei in graph[node]:
                if nei not in depth:
                    depth[nei] = depth[node] + 1
                    q.append(nei)

        max_depth = max(depth.values())

        MOD = 10**9 + 7
        # Number of valid assignments = 2^(max_depth - 1)
        return pow(2, max_depth - 1, MOD)
