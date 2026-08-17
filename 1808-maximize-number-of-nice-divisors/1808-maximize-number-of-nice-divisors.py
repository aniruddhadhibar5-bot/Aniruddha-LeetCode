MOD = 10**9 + 7

class Solution:
    def maxNiceDivisors(self, primeFactors):
        def mod_pow(base, exp):
            result = 1
            while exp > 0:
                if exp % 2:
                    result = result * base % MOD
                base = base * base % MOD
                exp //= 2
            return result

        if primeFactors <= 3:
            return primeFactors

        # Break primeFactors into 3’s
        q, r = divmod(primeFactors, 3)
        if r == 0:
            return mod_pow(3, q)
        elif r == 1:
            return mod_pow(3, q-1) * 4 % MOD
        else:  # r == 2
            return mod_pow(3, q) * 2 % MOD
