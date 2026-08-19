use std::cmp::max;

impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut max_distance = 0;
        let mut last_pos: Option<i32> = None;
        let mut current_pos = 0;

        while n > 0 {
            // Check if the current lowest bit is 1
            if n & 1 == 1 {
                if let Some(prev) = last_pos {
                    // Calculate distance to the previous adjacent 1
                    max_distance = max(max_distance, current_pos - prev);
                }
                // Update the position of the last seen 1
                last_pos = Some(current_pos);
            }
            
            // Shift right by 1 bit and advance position tracker
            n >>= 1;
            current_pos += 1;
        }

        max_distance
    }
}
