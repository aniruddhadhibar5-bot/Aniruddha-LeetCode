class Solution:
    def findLUSlength(self, strs):
        def is_subsequence(a, b):
            # check if a is subsequence of b
            i = 0
            for ch in b:
                if i < len(a) and a[i] == ch:
                    i += 1
            return i == len(a)

        strs.sort(key=len, reverse=True)  # longest first

        for i, s in enumerate(strs):
            uncommon = True
            for j, t in enumerate(strs):
                if i == j:
                    continue
                if len(t) < len(s):
                    break  # no need to check shorter strings
                if is_subsequence(s, t):
                    uncommon = False
                    break
            if uncommon:
                return len(s)
        return -1
