use std::cmp::{max, min};

impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        // 1. Sort the array to process partition splits linearly
        nums.sort_unstable();

        // Establish the baseline score if all elements move in the same direction
        let mut min_score = nums[n - 1] - nums[0];

        // 2. Evaluate splitting the array at every index i
        // Elements from 0 to i get +k, elements from i+1 to n-1 get -k
        for i in 0..n - 1 {
            let high = max(nums[i] + k, nums[n - 1] - k);
            let low = min(nums[0] + k, nums[i + 1] - k);
            
            min_score = min(min_score, high - low);
        }

        min_score
    }
}
