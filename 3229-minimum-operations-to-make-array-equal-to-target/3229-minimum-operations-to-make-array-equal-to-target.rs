impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut current_diff: i64 = 0;

        for i in 0..nums.len() {
            let d = (target[i] - nums[i]) as i64;
            
            // Check if the current difference has the same sign as the previous one
            if (d > 0 && current_diff > 0) || (d < 0 && current_diff < 0) {
                // If the magnitude increases, we need additional operations
                if d.abs() > current_diff.abs() {
                    ans += d.abs() - current_diff.abs();
                }
            } else {
                // If the sign changes or starts from zero, we need the full magnitude
                ans += d.abs();
            }
            
            current_diff = d;
        }

        ans
    }
}
