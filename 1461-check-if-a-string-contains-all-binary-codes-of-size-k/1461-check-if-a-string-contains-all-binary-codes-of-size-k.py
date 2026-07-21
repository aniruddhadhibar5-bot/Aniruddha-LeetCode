class Solution(object):
    def hasAllCodes(self, s, k):
        # 1. Total possible unique binary codes of length k
        total_required = 1 << k  # This is equivalent to 2^k
        
        # 2. Early exit if the string cannot physically hold enough substrings
        if len(s) - k + 1 < total_required:
            return False
            
        seen_codes = set()
        
        # 3. Slide a window of size k across the string
        for i in range(len(s) - k + 1):
            substring = s[i : i + k]
            seen_codes.add(substring)
            
            # Optimization: break early if we already found all required codes
            if len(seen_codes) == total_required:
                return True
                
        return len(seen_codes) == total_required
