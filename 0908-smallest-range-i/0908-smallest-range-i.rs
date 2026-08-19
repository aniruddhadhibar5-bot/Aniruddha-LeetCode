use std::cmp::max;

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        // Find the absolute minimum and maximum values in the array
        let min_val = *nums.iter().min().unwrap_or(&0);
        let max_val = *nums.iter().max().unwrap_or(&0);

        // The maximum reduction of the gap is 2 * k
        let initial_gap = max_val - min_val;
        
        // Return the remaining gap, capped at a minimum of 0
        max(0, initial_gap - 2 * k)
    }
}
