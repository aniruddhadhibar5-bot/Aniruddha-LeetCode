class Solution(object):
    def minPartitions(self, n):
        # The minimum number of deci-binary numbers needed 
        # is simply the largest digit character in the string.
        return int(max(n))
