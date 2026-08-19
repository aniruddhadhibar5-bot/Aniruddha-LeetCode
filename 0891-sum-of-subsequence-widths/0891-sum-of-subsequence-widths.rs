impl Solution {
    pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
        let r_mod = 1_000_000_007i64;
        let n = nums.len();
        
        // Sorting allows us to easily compute the number of elements smaller/larger than any element
        nums.sort_unstable();

        let mut total_width_sum = 0i64;
        let mut pow2 = 1i64; // Tracks 2^i % r_mod

        for i in 0..n {
            // Contribution of nums[i] as maximum minus its contribution as minimum from the symmetric opposite end
            let contribution = (nums[i] as i64 - nums[n - 1 - i] as i64) * pow2;
            total_width_sum = (total_width_sum + contribution) % r_mod;
            
            // Advance the power of 2 for the next iteration
            pow2 = (pow2 * 2) % r_mod;
        }

        // Rust's modulo can return a negative value if the intermediate result is negative, 
        // so we adjust to ensure a positive remainder.
        ((total_width_sum + r_mod) % r_mod) as i32
    }
}
