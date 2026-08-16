class Solution:
    def searchRange(self, nums, target):
        def find_bound(is_left):
            left, right = 0, len(nums) - 1
            bound = -1
            while left <= right:
                mid = (left + right) // 2
                if nums[mid] == target:
                    bound = mid
                    if is_left:
                        right = mid - 1  # keep searching left
                    else:
                        left = mid + 1   # keep searching right
                elif nums[mid] < target:
                    left = mid + 1
                else:
                    right = mid - 1
            return bound

        start = find_bound(True)
        end = find_bound(False)
        return [start, end]
