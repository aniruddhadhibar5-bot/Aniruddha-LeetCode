class Solution:
    def isPalindrome(self, s):
        # Keep only alphanumeric characters and convert to lowercase
        filtered = ''.join(ch.lower() for ch in s if ch.isalnum())
        # Check if the filtered string reads the same forward and backward
        return filtered == filtered[::-1]
