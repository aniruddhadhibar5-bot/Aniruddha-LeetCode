class Solution:
    def maxValue(self, nums):
        n = len(nums)
        ans = [0] * n
        
        # Step 1: Precalculate the running maximum from left to right
        preMax = [nums[0]] * n
        for i in range(1, n):
            preMax[i] = max(preMax[i - 1], nums[i])
            
        # Step 2: Iterate backward to propagate the maximum values
        sufMin = float('inf')
        for i in range(n - 1, -1, -1):
            if preMax[i] > sufMin:
                # Can reach the right sequence, inherit index i + 1's max reachable value
                ans[i] = ans[i + 1]
            else:
                # Cannot leap further right, limited to the prefix maximum up to i
                ans[i] = preMax[i]
                
            # Update the minimum seen so far from the right side
            sufMin = min(sufMin, nums[i])
            
        return ans
