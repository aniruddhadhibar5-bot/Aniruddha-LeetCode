class Solution(object):
    def convertToTitle(self, columnNumber):
        res = []
        while columnNumber > 0:
            columnNumber -= 1
            res.append(chr(columnNumber % 26 + ord('A')))
            columnNumber //= 26
        return "".join(reversed(res))
