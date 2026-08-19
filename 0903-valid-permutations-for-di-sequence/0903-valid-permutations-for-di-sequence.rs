impl Solution {
    pub fn num_perms_di_sequence(s: String) -> i32 {
        let r_mod = 1_000_000_007;
        let n = s.len();
        let bytes = s.as_bytes();

        // dp[j] stores the number of valid sequences of length i+1 
        // where the last element has relative rank j.
        let mut dp = vec![0; n + 1];
        
        // Base case: for a sequence of length 1 (index 0), there is 1 way
        dp[0] = 1;

        for i in 0..n {
            let mut next_dp = vec![0; n + 1];
            
            if bytes[i] == b'I' {
                // If Increasing, compute moving forward prefix sums
                let mut running_sum = 0;
                for k in 0..=i + 1 {
                    if k > 0 {
                        running_sum = (running_sum + dp[k - 1]) % r_mod;
                    }
                    next_dp[k] = running_sum;
                }
            } else {
                // If Decreasing, compute moving backward suffix sums
                let mut running_sum = 0;
                for k in (0..=i).rev() {
                    running_sum = (running_sum + dp[k]) % r_mod;
                    next_dp[k] = running_sum;
                }
            }
            dp = next_dp;
        }

        // Sum all possibilities for the final sequence of length n+1
        let mut total_permutations = 0;
        for j in 0..=n {
            total_permutations = (total_permutations + dp[j]) % r_mod;
        }

        total_permutations
    }
}
