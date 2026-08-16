class Solution:
    def subsetsWithDup(self, nums):
        res = []
        nums.sort()  # sort to group duplicates together

        def backtrack(start, path):
            res.append(path[:])
            for i in range(start, len(nums)):
                # skip duplicates
                if i > start and nums[i] == nums[i - 1]:
                    continue
                path.append(nums[i])
                backtrack(i + 1, path)
                path.pop()

        backtrack(0, [])
        return res
