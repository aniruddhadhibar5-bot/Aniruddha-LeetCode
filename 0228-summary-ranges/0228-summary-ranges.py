class Solution:
    def summaryRanges(self, nums):
        res = []
        if not nums:
            return res

        start = nums[0]
        for i in range(1, len(nums)):
            if nums[i] != nums[i - 1] + 1:
                # Close current range
                if start == nums[i - 1]:
                    res.append(str(start))
                else:
                    res.append("{}->{}".format(start, nums[i - 1]))
                start = nums[i]

        # Add the last range
        if start == nums[-1]:
            res.append(str(start))
        else:
            res.append("{}->{}".format(start, nums[-1]))

        return res
