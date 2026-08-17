class Solution:
    def kthFactor(self, n, k):
        factors = []
        for i in range(1, n + 1):
            if n % i == 0:
                factors.append(i)
        return factors[k-1] if k <= len(factors) else -1
