class Solution:
    def maxScore(self, nums):
        n = len(nums) // 2
        memo = {}

        # Custom gcd implementation
        def gcd(a, b):
            while b:
                a, b = b, a % b
            return a

        def dp(mask, step):
            if step > n:
                return 0
            if (mask, step) in memo:
                return memo[(mask, step)]
            best = 0
            for i in range(len(nums)):
                if mask & (1 << i):
                    continue
                for j in range(i+1, len(nums)):
                    if mask & (1 << j):
                        continue
                    new_mask = mask | (1 << i) | (1 << j)
                    score = step * gcd(nums[i], nums[j]) + dp(new_mask, step+1)
                    best = max(best, score)
            memo[(mask, step)] = best
            return best

        return dp(0, 1)
