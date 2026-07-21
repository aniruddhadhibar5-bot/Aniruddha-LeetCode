try:
    from math import gcd
except ImportError:
    from fractions import gcd

class Solution(object):
    def gcdSum(self, nums):
        n = len(nums)
        prefixGcd = [0] * n
        
        # 1. Construct the prefixGcd array
        current_max = 0
        for i in range(n):
            if nums[i] > current_max:
                current_max = nums[i]
            prefixGcd[i] = gcd(nums[i], current_max)
            
        # 2. Sort the array in non-decreasing order
        prefixGcd.sort()
        
        # 3. Pair the smallest and largest unpaired elements
        total_sum = 0
        left = 0
        right = n - 1
        
        while left < right:
            total_sum += gcd(prefixGcd[left], prefixGcd[right])
            left += 1
            right -= 1
            
        return total_sum
