class Solution(object):
    def concatenatedBinary(self, n):
        MOD = 10**9 + 7
        ans = 0
        
        # Loop from 1 to n to build the concatenated value dynamically
        for i in range(1, n + 1):
            # i.bit_length() gives the number of bits in the binary representation of i
            ans = ((ans << i.bit_length()) + i) % MOD
            
        return ans
