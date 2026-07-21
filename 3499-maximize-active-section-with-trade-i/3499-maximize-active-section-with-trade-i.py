import re

class Solution(object):
    def maxActiveSectionsAfterTrade(self, s):
        # 1. Count the original active sections
        initial_ones = s.count('1')
        
        # 2. Augment the string with '1' at both ends
        t = '1' + s + '1'
        
        # 3. Group the string into alternating blocks of '1's and '0's
        blocks = re.findall(r'1+|0+', t)
        
        # If there are fewer than 5 blocks, no internal '1' block is surrounded by '0's
        if len(blocks) < 5:
            return initial_ones
        
        max_total = initial_ones
        
        # 4. Iterate through internal '1' blocks (indices 2, 4, 6...)
        for i in range(2, len(blocks) - 2, 2):
            left_zeros = len(blocks[i - 1])
            right_zeros = len(blocks[i + 1])
            
            current_total = initial_ones + left_zeros + right_zeros
            max_total = max(max_total, current_total)
            
        return max_total
