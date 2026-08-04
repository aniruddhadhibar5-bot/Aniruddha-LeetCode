class Solution:
    def findMissingElements(self, nums):
        num_set = set(nums)
        return [x for x in range(min(nums) + 1, max(nums)) if x not in num_set]
