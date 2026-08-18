impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let mut island_count = 0;
        let rows = grid.len();
        let cols = grid[0].len(); // Fixed typo here as well (was grid.len())

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '1' {
                    island_count += 1;
                    // Trigger DFS to sink the entire connected island
                    Self::dfs(&mut grid, r, c, rows, cols);
                }
            }
        }

        island_count
    }

    // Removed the invalid "private" keyword. It is private by default.
    fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
        // Base case: boundary check or water cell
        if r >= rows || c >= cols || grid[r][c] == '0' {
            return;
        }

        // Sink the current land piece
        grid[r][c] = '0';

        // Explore all 4 orthogonal directions safely without usize underflow
        if r > 0 {
            Self::dfs(grid, r - 1, c, rows, cols);
        }
        Self::dfs(grid, r + 1, c, rows, cols);
        if c > 0 {
            Self::dfs(grid, r, c - 1, rows, cols);
        }
        Self::dfs(grid, r, c + 1, rows, cols);
    }
}
