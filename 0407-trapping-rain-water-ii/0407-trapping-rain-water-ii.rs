use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        if m == 0 { return 0; }
        let n = height_map[0].len();
        if n == 0 { return 0; }

        let mut visited = vec![vec![false; n]; m];
        // Min-heap elements are stored as: Reverse((height, row, col))
        let mut heap = BinaryHeap::new();

        // 1. Push all outermost boundary cells into the min-heap
        for r in 0..m {
            for c in 0..n {
                if r == 0 || r == m - 1 || c == 0 || c == n - 1 {
                    heap.push(Reverse((height_map[r][c], r, c)));
                    visited[r][c] = true;
                }
            }
        }

        let mut total_water = 0;
        let mut max_height = 0;
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        // 2. Process elements from lowest boundary inward
        while let Some(Reverse((h, r, c))) = heap.pop() {
            // Update the current trapping wall level limit
            max_height = max_height.max(h);

            // Explore 4 directional neighbors
            for &(dr, dc) in &dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if !visited[nr][nc] {
                        visited[nr][nc] = true;
                        let neighbor_h = height_map[nr][nc];
                        
                        // If neighbor is shorter than current spill line, it traps water
                        if neighbor_h < max_height {
                            total_water += max_height - neighbor_h;
                        }
                        
                        heap.push(Reverse((neighbor_h, nr, nc)));
                    }
                }
            }
        }

        total_water
    }
}
