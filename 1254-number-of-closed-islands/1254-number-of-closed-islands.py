class Solution:
    def closedIsland(self, grid):
        m, n = len(grid), len(grid[0])

        def dfs(i, j):
            # If out of bounds, island touches border → not closed
            if i < 0 or i >= m or j < 0 or j >= n:
                return False
            if grid[i][j] == 1:
                return True
            grid[i][j] = 1  # mark visited

            top = dfs(i - 1, j)
            bottom = dfs(i + 1, j)
            left = dfs(i, j - 1)
            right = dfs(i, j + 1)
            return top and bottom and left and right

        count = 0
        for i in range(m):
            for j in range(n):
                if grid[i][j] == 0 and dfs(i, j):
                    count += 1
        return count
