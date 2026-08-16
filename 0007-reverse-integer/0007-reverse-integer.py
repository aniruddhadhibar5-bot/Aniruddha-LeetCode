class Solution:
    def reverse(self, x: int) -> int:
        """
        Reverses the digits of a signed 32-bit integer.
        Time Complexity: O(log10(X)) - Number of digits in x.
        Space Complexity: O(1) - Constant auxiliary storage space.
        """
        # Define 32-bit signed integer boundary bounds
        INT_MIN, INT_MAX = -2**31, 2**31 - 1
        
        # Determine the sign multiplier and process using the absolute value
        sign = -1 if x < 0 else 1
        x = abs(x)
        
        reversed_num = 0
        while x != 0:
            # Extract the last digit
            digit = x % 10
            x //= 10
            
            # Step-by-step overflow check before mutating the number
            # This satisfies environments that do not allow 64-bit storage
            if (reversed_num > INT_MAX // 10) or (reversed_num == INT_MAX // 10 and digit > 7):
                return 0
                
            reversed_num = reversed_num * 10 + digit
            
        # Re-apply the original sign indicator
        final_result = sign * reversed_num
        
        # Final sanity check boundary filter
        if final_result < INT_MIN or final_result > INT_MAX:
            return 0
            
        return final_result
