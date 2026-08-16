class Solution:
    def merge(self, intervals):
        # Step 1: Sort intervals by start time
        intervals.sort(key=lambda x: x[0])
        merged = []

        for interval in intervals:
            # If merged is empty or no overlap, add interval
            if not merged or merged[-1][1] < interval[0]:
                merged.append(interval)
            else:
                # Overlap → merge by extending the end
                merged[-1][1] = max(merged[-1][1], interval[1])

        return merged
