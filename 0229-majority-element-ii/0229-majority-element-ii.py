class Solution:
    def majorityElement(self, nums):
        if not nums:
            return []

        # Step 1: Find candidates
        candidate1, candidate2 = None, None
        count1, count2 = 0, 0

        for num in nums:
            if candidate1 == num:
                count1 += 1
            elif candidate2 == num:
                count2 += 1
            elif count1 == 0:
                candidate1, count1 = num, 1
            elif count2 == 0:
                candidate2, count2 = num, 1
            else:
                count1 -= 1
                count2 -= 1

        # Step 2: Verify candidates
        result = []
        n = len(nums)
        for c in (candidate1, candidate2):
            if c is not None and nums.count(c) > n // 3:
                result.append(c)

        return result
