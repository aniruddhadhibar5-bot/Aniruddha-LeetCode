class Solution:
    def divide(self, dividend: int, divisor: int) -> int:
        # Define 32-bit signed integer limits
        INT_MIN = -2**31
        INT_MAX = 2**31 - 1
        
        # Handle the overflow edge case explicitly
        if dividend == INT_MIN and divisor == -1:
            return INT_MAX
            
        # Determine the sign of the resulting quotient
        # True if the signs are opposite, False if they match
        negative = (dividend < 0) ^ (divisor < 0)
        
        # Work with absolute values to simplify the bitwise subtraction
        a = abs(dividend)
        b = abs(divisor)
        
        quotient = 0
        
        # Exponentially subtract multiples of the divisor using bit shifting
        while a >= b:
            temp = b
            multiple = 1
            # Shift left to find the largest multiple that fits within the remainder
            while a >= (temp << 1):
                temp <<= 1
                multiple <<= 1
                
            a -= temp
            quotient += multiple
            
        # Apply the correct sign and clamp to 32-bit bounds
        if negative:
            quotient = -quotient
            
        return max(INT_MIN, min(INT_MAX, quotient))
