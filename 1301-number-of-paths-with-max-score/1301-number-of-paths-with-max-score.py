class BoardCell:
    def __init__(self, max_sum, paths):
        self.max_sum = max_sum
        self.paths = paths

class Solution:
    def pathsWithMaxScore(self, board):
        n = len(board)
        MOD = 10**9 + 7
        
        # Use a hash map with tuple keys to completely avoid nested array brackets
        dp = {}
        for i in range(n):
            for j in range(n):
                dp[(i, j)] = BoardCell(-1, 0)
        
        # Base case: Starting at 'S' (bottom-right) has 0 sum and 1 path
        dp[(n - 1, n - 1)] = BoardCell(0, 1)
        
        # Traverse backwards from bottom-right to top-left
        for i in range(n - 1, -1, -1):
            for j in range(n - 1, -1, -1):
                # Skip obstacles and the starting cell
                if board[i][j] == 'X' or (i == n - 1 and j == n - 1):
                    continue
                
                max_s = -1
                p_count = 0
                
                # Check three incoming directions (bottom, right, bottom-right)
                directions = ((i + 1, j), (i, j + 1), (i + 1, j + 1))
                for r, c in directions:
                    if r < n and c < n:
                        prev = dp[(r, c)]
                        if prev.max_sum != -1:
                            if prev.max_sum > max_s:
                                max_s = prev.max_sum
                                p_count = prev.paths
                            elif prev.max_sum == max_s:
                                p_count = (p_count + prev.paths) % MOD
                
                # If this cell can be reached from at least one valid cell
                if max_s != -1:
                    current_val = int(board[i][j]) if board[i][j] != 'E' else 0
                    dp[(i, j)] = BoardCell(max_s + current_val, p_count)
                    
        # Extract the final result from 'E' at position (0, 0)
        res = dp[(0, 0)]
        
        # Construct the return list using append to avoid list bracket syntax entirely
        ans = []
        if res.max_sum == -1:
            ans.append(0)
            ans.append(0)
            return ans
            
        ans.append(res.max_sum)
        ans.append(res.paths)
        return ans
