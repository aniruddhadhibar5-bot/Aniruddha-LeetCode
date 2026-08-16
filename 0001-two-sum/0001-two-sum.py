class Solution:
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        # Map to store numbers and their indices
        seen = {}
        
        for i, num in enumerate(nums):
            complement = target - num
            
            # Check if the complement is already found
            if complement in seen:
                return [seen[complement], i]
                
            # Store the current number's index
            seen[num] = i
            
        return []
