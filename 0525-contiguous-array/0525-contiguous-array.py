class Solution:
    def findMaxLength(self, nums):
        prefix_sum = 0
        seen = {0: -1}   # sum -> earliest index
        max_len = 0

        for i, num in enumerate(nums):
            prefix_sum += 1 if num == 1 else -1
            if prefix_sum in seen:
                max_len = max(max_len, i - seen[prefix_sum])
            else:
                seen[prefix_sum] = i
        return max_len
