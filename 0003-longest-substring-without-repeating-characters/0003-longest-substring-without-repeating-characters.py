class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        """
        Finds the length of the longest substring without repeating characters.
        Time Complexity: O(N) - Single pass through the string.
        Space Complexity: O(min(M, N)) - Map size is capped by string size N or alphabet size M.
        """
        # Map to store the last seen index of each character
        char_map = {}
        max_length = 0
        left = 0  # Left boundary of the sliding window
        
        for right, char in enumerate(s):
            # If char is already in the window, shrink the window by moving left
            if char in char_map and char_map[char] >= left:
                left = char_map[char] + 1
            
            # Update the last seen position of the character
            char_map[char] = right
            
            # Calculate and update the maximum length found so far
            max_length = max(max_length, right - left + 1)
            
        return max_length
