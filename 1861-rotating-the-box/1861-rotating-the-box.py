class Solution:
    def rotateTheBox(self, boxGrid):
        m = len(boxGrid)
        n = len(boxGrid[0])
        
        # Step 1: Let gravity take effect within each row (stones fall to the right)
        for i in range(m):
            empty_slot = n - 1  # Track the rightmost available position
            for j in range(n - 1, -1, -1):
                if boxGrid[i][j] == '*':
                    # Obstacle resets the falling boundary
                    empty_slot = j - 1
                elif boxGrid[i][j] == '#':
                    # Move stone to the empty slot
                    boxGrid[i][j] = '.'
                    boxGrid[i][empty_slot] = '#'
                    empty_slot -= 1
                    
        # Step 2: Rotate the grid 90 degrees clockwise into an n x m matrix
        rotated = [['.'] * m for _ in range(n)]
        for i in range(m):
            for j in range(n):
                rotated[j][m - 1 - i] = boxGrid[i][j]
                
        return rotated
