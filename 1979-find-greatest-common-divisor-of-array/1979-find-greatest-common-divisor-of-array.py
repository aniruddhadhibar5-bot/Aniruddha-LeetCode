try:
    from math import gcd
except ImportError:
    from fractions import gcd

class Solution(object):
    def findGCD(self, nums):
        # 1. Find the smallest and largest numbers in the array
        smallest = min(nums)
        largest = max(nums)
        
        # 2. Return their greatest common divisor
        return gcd(smallest, largest)
