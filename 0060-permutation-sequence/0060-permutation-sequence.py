class Solution:
    def getPermutation(self, n, k):
        import math
        nums = [str(i) for i in range(1, n + 1)]
        k -= 1  # convert to 0-index
        res = []

        for i in range(n, 0, -1):
            fact = math.factorial(i - 1)
            index = k // fact
            res.append(nums.pop(index))
            k %= fact

        return "".join(res)
