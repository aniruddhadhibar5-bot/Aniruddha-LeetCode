use std::cmp::max;

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut xy_area = 0;
        let mut zx_area = 0;
        let mut yz_area = 0;

        for i in 0..n {
            let mut max_in_row = 0;
            let mut max_in_col = 0;
            
            for j in 0..n {
                // Top projection: count non-zero cells
                if grid[i][j] > 0 {
                    xy_area += 1;
                }
                
                // Side projection: trace max height in row i
                max_in_row = max(max_in_row, grid[i][j]);
                
                // Front projection: trace max height in column i (by flipping index)
                max_in_col = max(max_in_col, grid[j][i]);
            }
            
            yz_area += max_in_row;
            zx_area += max_in_col;
        }

        xy_area + zx_area + yz_area
    }
}
