class Solution:
    def findSubsequences(self, nums):
        res = []

        def dfs(start, path):
            if len(path) >= 2:
                res.append(path[:])
            used = set()
            for i in range(start, len(nums)):
                if (path and nums[i] < path[-1]) or nums[i] in used:
                    continue
                used.add(nums[i])
                path.append(nums[i])
                dfs(i+1, path)
                path.pop()

        dfs(0, [])
        return res
