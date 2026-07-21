class Solution(object):
    def zigZagArrays(self, n, l, r):
        MOD = 10**9 + 7
        K = r - l + 1
        
        # Base case for an array of length 2
        # dp_up[v] tracks ways to reach 'v' via an increasing step
        # dp_down[v] tracks ways to reach 'v' via a decreasing step
        dp_up = [0] * (K + 1)
        dp_down = [0] * (K + 1)
        
        for v in range(1, K + 1):
            dp_up[v] = v - 1     # Count of valid choices u < v
            dp_down[v] = K - v   # Count of valid choices u > v
            
        # Transition from length 3 up to n
        for i in range(3, n + 1):
            next_up = [0] * (K + 1)
            next_down = [0] * (K + 1)
            
            # Optimize the summation for next_up using a prefix sum
            pref_sum = 0
            for v in range(1, K + 1):
                next_up[v] = pref_sum
                pref_sum = (pref_sum + dp_down[v]) % MOD
                
            # Optimize the summation for next_down using a suffix sum
            suff_sum = 0
            for v in range(K, 0, -1):
                next_down[v] = suff_sum
                suff_sum = (suff_sum + dp_up[v]) % MOD
                
            dp_up = next_up
            dp_down = next_down
            
        # The answer is the sum of all valid paths of length n
        return (sum(dp_up) + sum(dp_down)) % MOD
