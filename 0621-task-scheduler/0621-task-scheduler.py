from collections import Counter

class Solution:
    def leastInterval(self, tasks, n):
        freq = Counter(tasks)
        max_freq = max(freq.values())
        max_count = sum(1 for v in freq.values() if v == max_freq)

        return max(len(tasks), (max_freq - 1) * (n + 1) + max_count)
