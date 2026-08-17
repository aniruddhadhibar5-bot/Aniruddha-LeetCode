class Solution:
    def tilingRectangle(self, n, m):
        if n == m:
            return 1

        self.ans = n * m  # upper bound
        heights = [0] * m

        def dfs(count):
            # prune if already worse than best
            if count >= self.ans:
                return
            # find first column with minimal height
            min_h = min(heights)
            if min_h == n:
                self.ans = count
                return
            idx = heights.index(min_h)
            # find max square size that fits
            max_size = 1
            while idx + max_size <= m and all(heights[i] == min_h for i in range(idx, idx + max_size)) and min_h + max_size <= n:
                max_size += 1
            # try placing squares of all possible sizes
            for size in range(max_size - 1, 0, -1):
                for i in range(idx, idx + size):
                    heights[i] += size
                dfs(count + 1)
                for i in range(idx, idx + size):
                    heights[i] -= size

        dfs(0)
        return self.ans
