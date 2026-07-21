try:
    from math import gcd
except ImportError:
    from fractions import gcd

class Solution(object):
    def gcdOfOddEvenSums(self, n):
        sum_odd = n * n
        sum_even = n * (n + 1)
        return gcd(sum_odd, sum_even)
