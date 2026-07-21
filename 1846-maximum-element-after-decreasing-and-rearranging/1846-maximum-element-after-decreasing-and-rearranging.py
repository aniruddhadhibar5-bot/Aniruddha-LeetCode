class Solution(object):
    def maximumElementAfterDecrementingAndRearranging(self, arr):
        # 1. Sort the array to build an ascending sequence
        arr.sort()
        
        # 2. The first element must always be 1
        ans = 1
        
        # 3. Iterate through the rest of the elements
        for i in range(1, len(arr)):
            # If the current element is strictly greater than our current max,
            # we can increment our max by 1.
            if arr[i] > ans:
                ans += 1
                
        return ans
