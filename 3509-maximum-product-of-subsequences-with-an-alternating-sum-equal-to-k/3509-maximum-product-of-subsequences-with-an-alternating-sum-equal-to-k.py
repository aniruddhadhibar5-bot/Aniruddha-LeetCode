class Solution:
    def maxProduct(self, nums: list[int], k: int, limit: int) -> int:
        # --- Track 1: Non-zero subsequences with product constraints ---
        # dp maps (parity, current_sum) -> set of valid products
        dp = {}
        
        # --- Track 2: Pure reachability for zero-containing subsequences ---
        has_no_zero_sums = set()
        has_zero_sums = set()
        
        for x in nums:
            # 1. Update the constrained product DP (only for x > 0)
            if x > 0:
                next_dp = {}
                for state, products in dp.items():
                    next_dp[state] = set(products)
                    
                if x <= limit:
                    if (1, x) not in next_dp:
                        next_dp[(1, x)] = set()
                    next_dp[(1, x)].add(x)
                    
                for (parity, s), products in dp.items():
                    next_parity = 1 - parity
                    next_s = s + x if parity == 0 else s - x
                    for p in products:
                        np = p * x
                        if np <= limit:
                            if (next_parity, next_s) not in next_dp:
                                next_dp[(next_parity, next_s)] = set()
                            next_dp[(next_parity, next_s)].add(np)
                dp = next_dp

            # 2. Update the unconstrained reachability sets for zero tracking
            next_no_zero = set(has_no_zero_sums)
            next_zero = set(has_zero_sums)
            
            if x == 0:
                next_zero.add((1, 0))
                for (parity, s) in has_no_zero_sums:
                    next_zero.add((1 - parity, s))
                for (parity, s) in has_zero_sums:
                    next_zero.add((1 - parity, s))
            else:
                next_no_zero.add((1, x))
                for (parity, s) in has_no_zero_sums:
                    next_s = s + x if parity == 0 else s - x
                    next_no_zero.add((1 - parity, next_s))
                for (parity, s) in has_zero_sums:
                    next_s = s + x if parity == 0 else s - x
                    next_zero.add((1 - parity, next_s))
                    
            has_no_zero_sums = next_no_zero
            has_zero_sums = next_zero
            
        # --- Find the maximum valid outcome ---
        max_product = -1
        
        # Check valid products from the non-zero track
        for (parity, s), products in dp.items():
            if s == k:
                for p in products:
                    if p > max_product:
                        max_product = p
                        
        # Check if 0 is a valid option from the zero-containing track
        for (parity, s) in has_zero_sums:
            if s == k:
                if 0 > max_product:
                    max_product = 0
                    
        return max_product
