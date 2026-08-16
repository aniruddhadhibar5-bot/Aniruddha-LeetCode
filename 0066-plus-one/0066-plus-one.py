class Solution:
    def plusOne(self, digits):
        n = len(digits)

        # Traverse from the last digit backwards
        for i in range(n - 1, -1, -1):
            if digits[i] < 9:
                digits[i] += 1
                return digits
            digits[i] = 0

        # If all digits were 9, we need an extra digit at the front
        return [1] + digits
