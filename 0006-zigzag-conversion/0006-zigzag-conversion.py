class Solution:
    def convert(self, s: str, numRows: int) -> str:
        """
        Converts a string into a zigzag pattern layout and reads row-by-row.
        Time Complexity: O(N) - We visit each character in s exactly once.
        Space Complexity: O(N) - Storing characters in row buckets.
        """
        # Edge case: If 1 row or string is shorter than rows, no layout change happens
        if numRows == 1 or numRows >= len(s):
            return s
            
        # Initialize an array of strings representing each row bucket
        rows = ["" for _ in range(numRows)]
        current_row = 0
        going_down = False  # Track direction flag
        
        for char in s:
            rows[current_row] += char
            
            # Change direction when hitting top or bottom boundary rows
            if current_row == 0 or current_row == numRows - 1:
                going_down = not going_down
                
            # Move up or down based on direction flag
            current_row += 1 if going_down else -1
            
        # Join all row string buckets together
        return "".join(rows)
