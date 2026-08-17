class Solution:
    def findMin(self, nums):
        left, right = 0, len(nums) - 1

        while left < right:
            mid = (left + right) // 2
            if nums[mid] > nums[right]:
                # Minimum is in the right half
                left = mid + 1
            elif nums[mid] < nums[right]:
                # Minimum is in the left half
                right = mid
            else:
                # nums[mid] == nums[right], shrink search space
                right -= 1

        return nums[left]
