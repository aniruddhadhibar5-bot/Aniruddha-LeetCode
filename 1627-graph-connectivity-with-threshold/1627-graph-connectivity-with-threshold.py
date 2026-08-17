class Solution:
    def areConnected(self, n, threshold, queries):
        parent = list(range(n + 1))

        def find(x):
            if parent[x] != x:
                parent[x] = find(parent[x])
            return parent[x]

        def union(x, y):
            parent[find(x)] = find(y)

        # Connect cities sharing divisors > threshold
        for d in range(threshold + 1, n + 1):
            for multiple in range(2 * d, n + 1, d):
                union(d, multiple)

        # Answer queries
        res = []
        for a, b in queries:
            res.append(find(a) == find(b))
        return res
