use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        
        let grid_bytes: Vec<&[u8]> = grid.iter().map(|s| s.as_bytes()).collect();
        
        let mut start_r = 0;
        let mut start_c = 0;
        let mut total_keys = 0;

        // Scan the grid to find the start positions and count the number of keys
        for r in 0..m {
            for c in 0..n {
                let ch = grid_bytes[r][c];
                if ch == b'@' {
                    start_r = r;
                    start_c = c;
                } else if ch >= b'a' && ch <= b'f' {
                    total_keys += 1;
                }
            }
        }

        let target_mask = (1 << total_keys) - 1;
        
        // visited[r][c][keys_mask] keeps track of processed state combinations
        let mut visited = vec![vec![vec![false; 1 << total_keys]; n]; m];
        let mut queue = VecDeque::new();

        // Queue elements store: (r, c, keys_mask, steps)
        queue.push_back((start_r, start_c, 0, 0));
        visited[start_r][start_c][0] = true;

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some((r, c, mut mask, steps)) = queue.pop_front() {
            // If all keys are collected, return the total steps taken
            if mask == target_mask {
                return steps;
            }

            for &(dr, dc) in &directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                // Boundary verification
                if nr < 0 || nr >= m as i32 || nc < 0 || nc >= n as i32 {
                    continue;
                }

                let nr = nr as usize;
                let nc = nc as usize;
                let ch = grid_bytes[nr][nc];

                // Wall obstacle check
                if ch == b'#' {
                    continue;
                }

                // Lock check: check if we hold the matching key in our bitmask
                if ch >= b'A' && ch <= b'F' {
                    let lock_bit = (ch - b'A') as usize;
                    if (mask & (1 << lock_bit)) == 0 {
                        continue; // No key for this lock
                    }
                }

                // Key check: grab and update our bitmask if we land on a new key
                let mut next_mask = mask;
                if ch >= b'a' && ch <= b'f' {
                    let key_bit = (ch - b'a') as usize;
                    next_mask |= 1 << key_bit;
                }

                // If this state configuration is unvisited, traverse it
                if !visited[nr][nc][next_mask] {
                    visited[nr][nc][next_mask] = true;
                    queue.push_back((nr, nc, next_mask, steps + 1));
                }
            }
        }

        -1
    }
}
