use std::cmp::max;

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut total_area = 0;
        
        // Direction offsets for the 4 neighbors: Up, Down, Left, Right
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for r in 0..n {
            for c in 0..n {
                let v = grid[r][c];
                
                // If there are no cubes in this cell, it contributes 0 area
                if v == 0 {
                    continue;
                }

                // Add 2 for the top and bottom faces
                total_area += 2;

                // Check all 4 vertical sides
                for &(dr, dc) in &directions {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;

                    // If the neighbor is out of bounds, the entire height 'v' is exposed
                    if nr < 0 || nr >= n as i32 || nc < 0 || nc >= n as i32 {
                        total_area += v;
                    } else {
                        // Otherwise, only the height difference is exposed
                        let neighbor_v = grid[nr as usize][nc as usize];
                        total_area += max(0, v - neighbor_v);
                    }
                }
            }
        }

        total_area
    }
}
