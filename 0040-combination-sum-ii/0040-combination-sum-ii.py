class Solution:
    def combinationSum2(self, candidates, target):
        res = []
        candidates.sort()  # sort to handle duplicates

        def backtrack(start, path, total):
            if total == target:
                res.append(path[:])
                return
            if total > target:
                return

            prev = -1
            for i in range(start, len(candidates)):
                # skip duplicates at the same recursion depth
                if candidates[i] == prev:
                    continue
                path.append(candidates[i])
                backtrack(i + 1, path, total + candidates[i])  # move to next index
                path.pop()
                prev = candidates[i]

        backtrack(0, [], 0)
        return res
