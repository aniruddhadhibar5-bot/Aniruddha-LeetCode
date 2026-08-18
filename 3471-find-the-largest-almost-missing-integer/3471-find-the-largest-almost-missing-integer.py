class Solution:
    def largestInteger(self, nums, k):
        from collections import defaultdict

        seen = defaultdict(set)

        # Build subarray membership
        for i in range(len(nums) - k + 1):
            window = nums[i:i + k]
            for num in window:
                seen[num].add(i)

        # Collect candidates that appear in exactly one subarray
        candidates = [num for num, idxs in seen.items() if len(idxs) == 1]

        return max(candidates) if candidates else -1
