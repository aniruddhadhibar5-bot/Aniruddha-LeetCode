class Solution:
    def longestCommonPrefix(self, strs):
        if not strs:
            return ""

        # Start with the first string as the prefix
        prefix = strs[0]

        # Compare with each subsequent string
        for word in strs[1:]:
            while not word.startswith(prefix):
                prefix = prefix[:-1]
                if not prefix:
                    return ""
        return prefix
