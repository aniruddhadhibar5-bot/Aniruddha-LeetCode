from bisect import bisect_right

class Solution(object):
    def gcdValues(self, nums, queries):
        # 1. Get the maximum value to size our arrays
        max_val = max(nums)
        
        # 2. Count the frequencies of each number in nums
        freq = [0] * (max_val + 1)
        for x in nums:
            freq[x] += 1
            
        # 3. Count how many elements are multiples of each number i
        cnt = [0] * (max_val + 1)
        for i in range(1, max_val + 1):
            for j in range(i, max_val + 1, i):
                cnt[i] += freq[j]
                
        # 4. Compute exact number of pairs with GCD equal to i (Backward induction)
        exact_gcd = [0] * (max_val + 1)
        for i in range(max_val, 0, -1):
            total_pairs = (cnt[i] * (cnt[i] - 1)) // 2
            # Subtract counts of multiples
            sub = 0
            for j in range(2 * i, max_val + 1, i):
                sub += exact_gcd[j]
            exact_gcd[i] = total_pairs - sub
            
        # 5. Create a prefix sum array of the counts
        pref = [0] * (max_val + 1)
        for i in range(1, max_val + 1):
            pref[i] = pref[i - 1] + exact_gcd[i]
            
        # 6. Answer each query using binary search
        ans = []
        for q in queries:
            # Find the smallest GCD value 'idx' where pref[idx] > q
            idx = bisect_right(pref, q)
            ans.append(idx)
            
        return ans
