class Solution:
    def maxSubArray(self, nums):
        def helper(l, r):
            if l == r:
                return nums[l]
            mid = (l + r) // 2
            left = helper(l, mid)
            right = helper(mid + 1, r)

            # Find max crossing sum
            left_sum = float('-inf')
            curr = 0
            for i in range(mid, l - 1, -1):
                curr += nums[i]
                left_sum = max(left_sum, curr)

            right_sum = float('-inf')
            curr = 0
            for i in range(mid + 1, r + 1):
                curr += nums[i]
                right_sum = max(right_sum, curr)

            cross = left_sum + right_sum
            return max(left, right, cross)

        return helper(0, len(nums) - 1)
