MOD = 10**9 + 7

class Solution:
    def waysToFillArray(self, queries):
        MAX = 20000  # enough for n + exponent
        fact = [1] * (MAX + 1)
        inv_fact = [1] * (MAX + 1)

        # Precompute factorials and inverses
        for i in range(1, MAX + 1):
            fact[i] = fact[i-1] * i % MOD
        inv_fact[MAX] = pow(fact[MAX], MOD-2, MOD)
        for i in range(MAX, 0, -1):
            inv_fact[i-1] = inv_fact[i] * i % MOD

        def nCr(n, r):
            if r < 0 or r > n:
                return 0
            return fact[n] * inv_fact[r] % MOD * inv_fact[n-r] % MOD

        def prime_factors(x):
            factors = {}
            d = 2
            while d * d <= x:
                while x % d == 0:
                    factors[d] = factors.get(d, 0) + 1
                    x //= d
                d += 1
            if x > 1:
                factors[x] = factors.get(x, 0) + 1
            return factors

        res = []
        for n, k in queries:
            factors = prime_factors(k)
            ways = 1
            for e in factors.values():
                ways = ways * nCr(e + n - 1, n - 1) % MOD
            res.append(ways)
        return res
