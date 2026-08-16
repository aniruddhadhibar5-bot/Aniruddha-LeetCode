class Solution:
    def removeDuplicates(self, nums):
        if not nums:
            return 0

        # Pointer for the next unique position
        k = 1
        for i in range(1, len(nums)):
            if nums[i] != nums[i - 1]:
                nums[k] = nums[i]
                k += 1

        return k
