class Solution:
    def subsets(self, nums):
        res = []

        def backtrack(start, path):
            # Add current subset
            res.append(path[:])
            # Explore further
            for i in range(start, len(nums)):
                path.append(nums[i])
                backtrack(i + 1, path)
                path.pop()

        backtrack(0, [])
        return res
