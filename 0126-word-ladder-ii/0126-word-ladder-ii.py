from collections import defaultdict

class Solution:
    def findLadders(self, beginWord, endWord, wordList):
        wordSet = set(wordList)
        if endWord not in wordSet:
            return []

        parents = defaultdict(set)
        level = {beginWord}
        while level and endWord not in parents:
            next_level = defaultdict(set)
            for word in level:
                for i in range(len(word)):
                    for c in 'abcdefghijklmnopqrstuvwxyz':
                        new_word = word[:i] + c + word[i+1:]
                        if new_word in wordSet:
                            next_level[new_word].add(word)
            wordSet -= set(next_level.keys())
            parents.update(next_level)
            level = set(next_level.keys())

        res = []
        def dfs(word, path):
            if word == beginWord:
                res.append(path[::-1])
                return
            for p in parents[word]:
                dfs(p, path + [p])

        if endWord in parents:
            dfs(endWord, [endWord])
        return res
