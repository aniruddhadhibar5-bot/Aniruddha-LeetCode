class Solution(object):
    def binaryGap(self, n):
        max_dist = 0
        last_pos = -1
        current_pos = 0
        
        # Process every bit of the integer n
        while n > 0:
            # Check if the lowest bit is a 1
            if n & 1:
                if last_pos != -1:
                    # Update maximum distance between adjacent 1s
                    max_dist = max(max_dist, current_pos - last_pos)
                last_pos = current_pos
                
            # Shift right to inspect the next bit position
            n >>= 1
            current_pos += 1
            
        return max_dist
