class Solution:
    def countDifferentSubsequenceGCDs(self, nums):
        max_num = max(nums)
        present = [False] * (max_num + 1)
        for num in nums:
            present[num] = True

        ans = 0
        for x in range(1, max_num + 1):
            g = 0
            for multiple in range(x, max_num + 1, x):
                if present[multiple]:
                    g = self.gcd(g, multiple)
                    if g == x:
                        ans += 1
                        break
        return ans

    def gcd(self, a, b):
        while b:
            a, b = b, a % b
        return a
