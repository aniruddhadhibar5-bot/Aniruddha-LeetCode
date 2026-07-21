class Solution(object):
    def sequentialDigits(self, low, high):
        result = []
        digits = "123456789"
        
        # Sequential numbers can have lengths from 2 digits up to 9 digits
        for length in range(2, 10):
            # Move our window across the "123456789" string
            for start in range(10 - length):
                substring = digits[start : start + length]
                num = int(substring)
                
                # Check if it fits within the valid range
                if low <= num <= high:
                    result.append(num)
                    
        # The numbers are generated in increasing order naturally, 
        # but sorting guarantees correctness regardless of the loop structure.
        result.sort()
        return result
