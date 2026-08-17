class Solution:
    def reverseWords(self, s):
        # Remove leading/trailing spaces, split by whitespace
        words = s.strip().split()
        # Reverse the list of words and join with single spaces
        return " ".join(reversed(words))
