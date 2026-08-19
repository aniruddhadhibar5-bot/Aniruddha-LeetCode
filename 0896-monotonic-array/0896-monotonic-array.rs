impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut is_increasing = true;
        let mut is_decreasing = true;

        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                is_increasing = false;
            }
            if nums[i] < nums[i + 1] {
                is_decreasing = false;
            }

            // Early exit optimization
            if !is_increasing && !is_decreasing {
                return false;
            }
        }

        true
    }
}
