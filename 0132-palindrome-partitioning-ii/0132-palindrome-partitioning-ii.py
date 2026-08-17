class Solution:
    def minCut(self, s):
        n = len(s)
        dp = [0] * (n + 1)
        for i in range(n + 1):
            dp[i] = i - 1  # worst case: cut between every character

        is_pal = [[False] * n for _ in range(n)]

        # Fill palindrome table from end to start
        for i in range(n - 1, -1, -1):
            for j in range(i, n):
                if s[i] == s[j] and (j - i < 2 or is_pal[i + 1][j - 1]):
                    is_pal[i][j] = True

        # Compute minimum cuts
        for i in range(1, n + 1):
            for j in range(i):
                if is_pal[j][i - 1]:
                    dp[i] = min(dp[i], dp[j] + 1)

        return dp[n]
