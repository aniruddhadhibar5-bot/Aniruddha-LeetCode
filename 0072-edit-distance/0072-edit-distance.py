class Solution:
    def minDistance(self, word1, word2):
        m, n = len(word1), len(word2)

        # DP table: (m+1) x (n+1)
        dp = [[0] * (n + 1) for _ in range(m + 1)]

        # Base cases
        for i in range(m + 1):
            dp[i][0] = i   # delete all characters from word1
        for j in range(n + 1):
            dp[0][j] = j   # insert all characters into word1

        # Fill DP table
        for i in range(1, m + 1):
            for j in range(1, n + 1):
                if word1[i - 1] == word2[j - 1]:
                    dp[i][j] = dp[i - 1][j - 1]  # no operation needed
                else:
                    dp[i][j] = 1 + min(
                        dp[i - 1][j],     # delete
                        dp[i][j - 1],     # insert
                        dp[i - 1][j - 1]  # replace
                    )

        return dp[m][n]
