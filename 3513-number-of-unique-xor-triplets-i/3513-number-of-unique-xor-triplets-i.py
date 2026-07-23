class Solution(object):
    def uniqueXorTriplets(self, nums):
        n = len(nums)
        
        # Base cases for small arrays
        if n < 3:
            return n
            
        # Total unique values is the next highest power of 2
        return 1 << (n).bit_length()
