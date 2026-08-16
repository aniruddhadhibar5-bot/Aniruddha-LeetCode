class Solution:
    def longestPalindrome(self, s: str) -> str:
        """
        Finds the longest palindromic substring by expanding around potential centers.
        Time Complexity: O(N^2) - N centers, expanding up to N/2 times for each.
        Space Complexity: O(1) - Modifies tracking boundaries inline.
        """
        if not s or len(s) < 1:
            return ""
            
        start, end = 0, 0
        
        def expand_around_center(left: int, right: int) -> int:
            # Expand outwards as long as characters match and indices stay in bounds
            while left >= 0 and right < len(s) and s[left] == s[right]:
                left -= 1
                right += 1
            # Return length of the valid palindrome found
            return right - left - 1

        for i in range(len(s)):
            # Case 1: Odd-length palindromes (single character center)
            len1 = expand_around_center(i, i)
            # Case 2: Even-length palindromes (space between two characters center)
            len2 = expand_around_center(i, i + 1)
            
            # Find the longest palindrome from this center point
            max_len = max(len1, len2)
            
            # If a new maximum length is found, update the substring tracking bounds
            if max_len > (end - start):
                start = i - (max_len - 1) // 2
                end = i + max_len // 2
                
        return s[start:end + 1]
