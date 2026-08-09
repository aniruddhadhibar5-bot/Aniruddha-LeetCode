class Solution:
    def stoneGameII(self, piles):
        n = len(piles)
        
        suffix_sums = [0] * (n + 1)
        for i in range(n - 1, -1, -1):
            suffix_sums[i] = suffix_sums[i + 1] + piles[i]
            
        memo = {}
        
        def dp(i, m):
            if i >= n:
                return 0
            if i + 2 * m >= n:
                return suffix_sums[i]
                
            if (i, m) in memo:
                return memo[(i, m)]
                
            max_stones = 0
            for x in range(1, 2 * m + 1):
                max_stones = max(max_stones, suffix_sums[i] - dp(i + x, max(m, x)))
                
            memo[(i, m)] = max_stones
            return max_stones

        return dp(0, 1)
