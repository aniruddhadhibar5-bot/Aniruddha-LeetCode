class Solution:
    def letterCombinations(self, digits):
        if not digits:
            return []

        phone_map = {
            '2': 'abc', '3': 'def', '4': 'ghi', '5': 'jkl',
            '6': 'mno', '7': 'pqrs', '8': 'tuv', '9': 'wxyz'
        }

        res = ['']
        for digit in digits:
            res = [prefix + letter for prefix in res for letter in phone_map[digit]]

        return res
