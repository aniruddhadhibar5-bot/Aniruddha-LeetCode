class TrieNode:
    def __init__(self):
        # Stores links to children nodes (a-z)
        self.children = {}
        # Marks the end of a complete word
        self.is_end_of_word = False

class WordDictionary:
    def __init__(self):
        """
        Initializes the data structure.
        """
        self.root = TrieNode()

    def addWord(self, word):
        """
        Adds a word into the data structure.
        """
        current = self.root
        for char in word:
            if char not in current.children:
                current.children[char] = TrieNode()
            current = current.children[char]
        current.is_end_of_word = True

    def search(self, word):
        """
        Returns true if the word matches any previously added string.
        The word may contain dots '.' to match any single letter.
        """
        # Helper function to perform DFS recursive search
        def dfs(index, node):
            current = node
            
            for i in range(index, len(word)):
                char = word[i]
                
                if char == '.':
                    # Wildcard: check every possible matching child node path
                    for child in current.children.values():
                        if dfs(i + 1, child):
                            return True
                    return False
                else:
                    # Standard Trie lookup
                    if char not in current.children:
                        return False
                    current = current.children[char]
                    
            return current.is_end_of_word

        return dfs(0, self.root)
