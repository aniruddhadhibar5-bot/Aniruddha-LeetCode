class TrieNode:
    def __init__(self):
        # Stores links to children nodes (a-z)
        self.children = {}
        # Marks the end of a complete word
        self.is_end_of_word = False

class Trie:
    def __init__(self):
        """
        Initializes your data structure here.
        """
        self.root = TrieNode()

    def insert(self, word):
        """
        Inserts a word into the trie.
        """
        current = self.root
        for char in word:
            if char not in current.children:
                current.children[char] = TrieNode()
            current = current.children[char]
        current.is_end_of_word = True

    def search(self, word):
        """
        Returns true if the word is in the trie.
        """
        current = self.root
        for char in word:
            if char not in current.children:
                return False
            current = current.children[char]
        return current.is_end_of_word

    def startsWith(self, prefix):
        """
        Returns true if there is any word in the trie that starts with the given prefix.
        """
        current = self.root
        for char in prefix:
            if char not in current.children:
                return False
            current = current.children[char]
        return True
