import math
from bisect import bisect_left, bisect_right

class Solution:
    def maxActiveSectionsAfterTrade(self, s: str, queries: list[list[int]]) -> list[int]:
        n = len(s)
        orig_ones = s.count('1')
        
        # 1. Identify all standalone '1' blocks in s
        start_1, end_1 = [], []
        i = 0
        while i < n:
            if s[i] == '1':
                start = i
                while i < n and s[i] == '1':
                    i += 1
                start_1.append(start)
                end_1.append(i - 1)
            else:
                i += 1
                
            
        M = len(start_1)
        if M == 0:
            return [orig_ones] * len(queries)
            
        # 2. Extract block sizes
        len_1 = [end_1[idx] - start_1[idx] + 1 for idx in range(M)]
        full_B = [start_1[idx+1] - end_1[idx] - 1 for idx in range(M - 1)]
        pair_B = [full_B[idx] + full_B[idx+1] for idx in range(M - 2)]
            
        # 3. Build Sparse Tables for O(1) Range Queries
        def build_st(arr, op):
            if not arr: return []
            size = len(arr)
            k = int(math.log2(size)) + 1
            st = [[0] * k for _ in range(size)]
            for idx in range(size):
                st[idx][0] = arr[idx]
            for j in range(1, k):
                for idx in range(size - (1 << j) + 1):
                    st[idx][j] = op(st[idx][j-1], st[idx + (1 << (j-1))][j-1])
            return st

        st_A_min = build_st(len_1, min)
        st_B_max = build_st(full_B, max)
        st_pair_B_max = build_st(pair_B, max)
        
        def query_st(st, L, R, op):
            if L > R: return None
            j = int(math.log2(R - L + 1))
            return op(st[L][j], st[R - (1 << j) + 1][j])
            
        # 4. Process Queries
        ans = []
        for l, r in queries:
            # Find internal '1' blocks strictly inside [l, r]
            i_start = bisect_right(start_1, l)
            i_end = bisect_left(end_1, r) - 1
            
            if i_start > i_end:
                ans.append(orig_ones)
                continue
                
            k_blocks = i_end - i_start + 1
            
            # Calculate lengths of boundary '0' blocks (B0 and Bk)
            start_of_B0 = end_1[i_start - 1] + 1 if i_start > 0 else 0
            B0 = start_1[i_start] - max(start_of_B0, l)
            
            end_of_Bk = start_1[i_end + 1] - 1 if i_end + 1 < M else n - 1
            Bk = min(end_of_Bk, r) - end_1[i_end]
            
            if k_blocks == 1:
                max_gain = B0 + Bk
            else:
                # Query maximum intermediate full '0' block
                max_full_B = query_st(st_B_max, i_start, i_end - 1, max)
                candidates_B = [B0, Bk]
                if max_full_B is not None:
                    candidates_B.append(max_full_B)
                M_B = max(candidates_B)
                
                # Scenario 2: Flip the smallest '1' block and the largest '0' block
                min_A = query_st(st_A_min, i_start, i_end, min)
                gain_type2 = M_B - min_A
                
                # Scenario 1: Flip an adjacent pair of '0' blocks
                gain_type1_left = B0 + full_B[i_start]
                gain_type1_right = full_B[i_end - 1] + Bk
                max_pair_B = query_st(st_pair_B_max, i_start, i_end - 2, max)
                
                candidates_gain = [gain_type2, gain_type1_left, gain_type1_right]
                if max_pair_B is not None:
                    candidates_gain.append(max_pair_B)
                    
                max_gain = max(candidates_gain)
                
            ans.append(orig_ones + max_gain)
            
        return ans
