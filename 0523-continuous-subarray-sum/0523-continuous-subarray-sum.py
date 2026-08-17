class Solution:
    def checkSubarraySum(self, nums, k):
        prefix_mod = {0: -1}  # remainder -> earliest index
        total = 0

        for i, num in enumerate(nums):
            total += num
            remainder = total % k
            if remainder in prefix_mod:
                if i - prefix_mod[remainder] >= 2:
                    return True
            else:
                prefix_mod[remainder] = i
        return False
