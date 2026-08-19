use std::collections::HashSet;
use std::cmp::max;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        // Store obstacles in a HashSet for O(1) average lookup time
        let mut obstacle_set = HashSet::with_capacity(obstacles.len());
        for obs in obstacles {
            if obs.len() == 2 {
                obstacle_set.insert((obs[0], obs[1]));
            }
        }

        // Direction vectors mapping to: North, East, South, West
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut dir_idx = 0; // Starts facing North

        let mut x = 0;
        let mut y = 0;
        let mut max_dist_sq = 0;

        for cmd in commands {
            if cmd == -1 {
                // Turn right 90 degrees
                dir_idx = (dir_idx + 1) % 4;
            } else if cmd == -2 {
                // Turn left 90 degrees
                dir_idx = (dir_idx + 3) % 4;
            } else {
                // Move forward step-by-step
                let (dx, dy) = directions[dir_idx];
                
                for _ in 0..cmd {
                    let next_x = x + dx;
                    let next_y = y + dy;

                    // Stop moving forward if an obstacle blocks the path
                    if obstacle_set.contains(&(next_x, next_y)) {
                        break;
                    }

                    x = next_x;
                    y = next_y;
                    
                    // Track maximum squared Euclidean distance
                    max_dist_sq = max(max_dist_sq, x * x + y * y);
                }
            }
        }

        max_dist_sq
    }
}
