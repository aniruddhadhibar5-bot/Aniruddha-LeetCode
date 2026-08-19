impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            if nums[left] % 2 == 0 {
                // Already even, keep it at the front
                left += 1;
            } else if nums[right] % 2 != 0 {
                // Already odd, keep it at the back
                right -= 1;
            } else {
                // Swap the misplaced odd and even elements
                nums.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        nums
    }
}
