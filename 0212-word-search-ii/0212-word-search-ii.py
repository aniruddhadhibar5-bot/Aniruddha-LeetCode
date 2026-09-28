class TrieNode:
    def __init__(self):
        self.children = {}
        self.word = None  # Stores the full word when a complete word is formed

class Solution:
    def findWords(self, board, words):
        # Step 1: Build the Trie
        root = TrieNode()
        for word in words:
            current = root
            for char in word:
                if char not in current.children:
                    current.children[char] = TrieNode()
                current = current.children[char]
            current.word = word  # Label the end node with the full string
            
        rows, cols = len(board), len(board[0])
        result = []
        
        # Step 2: Backtracking function
        def backtracking(r, c, parent_node):
            char = board[r][c]
            current_node = parent_node.children[char]
            
            # If we found a valid word, add it to our results
            if current_node.word is not None:
                result.append(current_node.word)
                current_node.word = None  # Clear to avoid duplicating this word
                
            # Mark the current cell as visited
            board[r][c] = '#'
            
            # Explore all 4 possible neighbor directions (Up, Down, Left, Right)
            for dr, dc in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                nr, nc = r + dr, c + dc
                if 0 <= nr < rows and 0 <= nc < cols:
                    next_char = board[nr][nc]
                    if next_char in current_node.children:
                        backtracking(nr, nc, current_node)
                        
            # Restore the cell character (backtrack step)
            board[r][c] = char
            
            # Optimization: Prune leaf nodes from the Trie to minimize further search spaces
            if not current_node.children:
                parent_node.children.pop(char)

        # Step 3: Loop through every cell on the board
        for r in range(rows):
            for c in range(cols):
                if board[r][c] in root.children:
                    backtracking(r, c, root)
                    
        return result
