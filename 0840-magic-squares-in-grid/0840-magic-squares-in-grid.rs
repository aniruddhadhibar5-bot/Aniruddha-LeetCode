impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        
        // A 3x3 magic square requires at least a 3x3 grid area
        if rows < 3 || cols < 3 {
            return 0;
        }
        
        let mut count = 0;
        
        // Check every possible 3x3 subgrid by its top-left corner (r, c)
        for r in 0..=(rows - 3) {
            for c in 0..=(cols - 3) {
                if Self::is_magic_square(&grid, r, c) {
                    count += 1;
                }
            }
        }
        
        count
    }
    
    fn is_magic_square(grid: &Vec<Vec<i32>>, r: usize, c: usize) -> bool {
        // Optimization: The center element of a 3x3 magic square must always be 5
        if grid[r + 1][c + 1] != 5 {
            return false;
        }
        
        // Track the presence of numbers 1-9 using a boolean array (or bitmask)
        let mut seen = [false; 10];
        
        for i in 0..3 {
            for j in 0..3 {
                let num = grid[r + i][c + j];
                if num < 1 || num > 9 || seen[num as usize] {
                    return false;
                }
                seen[num as usize] = true;
            }
        }
        
        // Row sums
        let r0 = grid[r][c] + grid[r][c + 1] + grid[r][c + 2];
        let r1 = grid[r + 1][c] + grid[r + 1][c + 1] + grid[r + 1][c + 2];
        let r2 = grid[r + 2][c] + grid[r + 2][c + 1] + grid[r + 2][c + 2];
        if r0 != 15 || r1 != 15 || r2 != 15 { return false; }
        
        // Column sums
        let c0 = grid[r][c] + grid[r + 1][c] + grid[r + 2][c];
        let c1 = grid[r][c + 1] + grid[r + 1][c + 1] + grid[r + 2][c + 1];
        let c2 = grid[r][c + 2] + grid[r + 1][c + 2] + grid[r + 2][c + 2];
        if c0 != 15 || c1 != 15 || c2 != 15 { return false; }
        
        // Diagonal sums
        let d0 = grid[r][c] + grid[r + 1][c + 1] + grid[r + 2][c + 2];
        let d1 = grid[r][c + 2] + grid[r + 1][c + 1] + grid[r + 2][c];
        if d0 != 15 || d1 != 15 { return false; }
        
        true
    }
}
