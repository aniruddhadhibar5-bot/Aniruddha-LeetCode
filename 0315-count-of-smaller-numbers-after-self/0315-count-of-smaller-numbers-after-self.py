class Solution(object):
    def countSmaller(self, nums):
        n = len(nums)
        res = [0] * n
        # Pair each number with its index
        enum = list(enumerate(nums))

        def merge_sort(arr):
            if len(arr) <= 1:
                return arr
            mid = len(arr) // 2
            left = merge_sort(arr[:mid])
            right = merge_sort(arr[mid:])
            merged = []
            i = j = 0
            # Count smaller elements
            while i < len(left) and j < len(right):
                if left[i][1] <= right[j][1]:
                    merged.append(right[j])
                    j += 1
                else:
                    # All remaining elements in right are smaller
                    res[left[i][0]] += len(right) - j
                    merged.append(left[i])
                    i += 1
            merged.extend(left[i:])
            merged.extend(right[j:])
            return merged

        merge_sort(enum)
        return res
