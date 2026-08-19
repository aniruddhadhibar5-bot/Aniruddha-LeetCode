impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        
        // The first column will entirely consist of 1s after optimal row flips.
        // Each 1 in the first column contributes 2^(n-1) to the total score.
        let mut max_score = m as i32 * (1 << (n - 1));

        // Evaluate columns from index 1 to n-1
        for j in 1..n {
            let mut count_ones = 0;
            
            for i in 0..m {
                // If the current element matches the first element of its row, 
                // it will effectively become a 1 after the initial row flips.
                if grid[i][j] == grid[i][0] {
                    count_ones += 1;
                }
            }
            
            // Greedily choose whether to flip the column to maximize the number of 1s
            let optimal_ones = count_ones.max(m - count_ones);
            
            // Accumulate the column's contribution to the score
            max_score += optimal_ones as i32 * (1 << (n - 1 - j));
        }

        max_score
    }
}
