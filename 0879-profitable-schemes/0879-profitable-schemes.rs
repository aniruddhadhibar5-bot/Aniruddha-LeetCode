use std::cmp::min;

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let r_mod = 1_000_000_007;
        let n = n as usize;
        let min_profit = min_profit as usize;
        
        // dp[i][j] represents the number of schemes using i members with AT LEAST j profit
        let mut dp = vec![vec![0; min_profit + 1]; n + 1];
        
        // Base case: 1 way to choose zero crimes (0 members, 0 profit)
        dp[0][0] = 1;

        for k in 0..group.len() {
            let members_needed = group[k] as usize;
            let p = profit[k] as usize;
            
            // Loop backwards to use the same 2D array layers securely in-place
            for i in (members_needed..=n).rev() {
                for j in (0..=min_profit).rev() {
                    // Cap the combined profit at min_profit
                    let next_profit = min(min_profit, j + p);
                    
                    dp[i][next_profit] = (dp[i][next_profit] + dp[i - members_needed][j]) % r_mod;
                }
            }
        }

        // Sum up all valid schemes that achieved at least min_profit across any member count <= n
        let mut total_schemes = 0;
        for i in 0..=n {
            total_schemes = (total_schemes + dp[i][min_profit]) % r_mod;
        }

        total_schemes
    }
}
