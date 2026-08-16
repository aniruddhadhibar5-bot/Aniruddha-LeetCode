class Solution:
    def insert(self, intervals, newInterval):
        res = []
        i = 0
        n = len(intervals)

        # Step 1: Add all intervals that end before newInterval starts
        while i < n and intervals[i][1] < newInterval[0]:
            res.append(intervals[i])
            i += 1

        # Step 2: Merge overlapping intervals with newInterval
        while i < n and intervals[i][0] <= newInterval[1]:
            newInterval[0] = min(newInterval[0], intervals[i][0])
            newInterval[1] = max(newInterval[1], intervals[i][1])
            i += 1
        res.append(newInterval)

        # Step 3: Add remaining intervals
        while i < n:
            res.append(intervals[i])
            i += 1

        return res
