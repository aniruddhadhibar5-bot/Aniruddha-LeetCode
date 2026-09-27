class Solution:
    def titleToNumber(self, columnTitle: str) -> int:
        result = 0
        for char in columnTitle:
            # Shift the value by base 26 and add the current letter's weight
            value = ord(char) - ord('A') + 1
            result = result * 26 + value
        return result
