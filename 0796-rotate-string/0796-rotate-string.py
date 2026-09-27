class Solution:
    def rotateString(self, s, goal):
        # If lengths are different, s can never be shifted to match goal
        if len(s) != len(goal):
            return False
        
        # Check if goal is a substring of s concatenated with itself
        return goal in (s + s)
