class Solution:
    def countDigitOne(self, n):
        count = 0
        factor = 1  # digit place: 1, 10, 100, ...
        
        while factor <= n:
            high = n // (factor * 10)
            current = (n // factor) % 10
            low = n % factor

            if current == 0:
                count += high * factor
            elif current == 1:
                count += high * factor + low + 1
            else:
                count += (high + 1) * factor

            factor *= 10

        return count
