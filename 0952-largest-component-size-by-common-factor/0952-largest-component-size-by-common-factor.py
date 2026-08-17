class Solution:
    def largestComponentSize(self, nums):
        parent = {}

        def find(x):
            if x not in parent:
                parent[x] = x
            if parent[x] != x:
                parent[x] = find(parent[x])
            return parent[x]

        def union(x, y):
            parent.setdefault(x, x)
            parent.setdefault(y, y)
            parent[find(x)] = find(y)

        def prime_factors(n):
            factors = set()
            d = 2
            while d * d <= n:
                while n % d == 0:
                    factors.add(d)
                    n //= d
                d += 1
            if n > 1:
                factors.add(n)
            return factors

        # Union each number with its prime factors
        for num in nums:
            for f in prime_factors(num):
                union(num, f)

        # Count sizes of connected components
        count = {}
        for num in nums:
            root = find(num)
            count[root] = count.get(root, 0) + 1

        return max(count.values())
