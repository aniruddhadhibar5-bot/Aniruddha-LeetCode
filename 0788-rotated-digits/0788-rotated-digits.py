class Solution:
    def rotatedDigits(self, n):
        count = 0
        
        # Define digit categories using sets for rapid checks
        invalid_digits = {'3', '4', '7'}
        rotating_digits = {'2', '5', '6', '9'}
        
        for i in range(1, n + 1):
            s = str(i)
            chars = set(s)
            
            # Condition 1: Must NOT contain any invalid digits
            if chars.intersection(invalid_digits):
                continue
                
            # Condition 2: Must contain AT LEAST ONE rotating digit
            if chars.intersection(rotating_digits):
                count += 1
                
        return count
