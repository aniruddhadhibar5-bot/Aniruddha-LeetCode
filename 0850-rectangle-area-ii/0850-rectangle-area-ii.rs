impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        let r_mod = 1_000_000_007i64;
        
        // 1. Collect all unique X coordinates
        let mut x_coords = Vec::with_capacity(rectangles.len() * 2);
        for r in &rectangles {
            x_coords.push(r[0]);
            x_coords.push(r[2]);
        }
        x_coords.sort_unstable();
        x_coords.dedup();

        let mut total_area: i64 = 0;

        // 2. Iterate through every adjacent pair of X coordinates (vertical strips)
        for i in 0..x_coords.len() - 1 {
            let x1 = x_coords[i];
            let x2 = x_coords[i + 1];
            let width = (x2 - x1) as i64;
            
            if width == 0 {
                continue;
            }

            // 3. Find all Y intervals for rectangles that span horizontally across this strip
            let mut y_intervals = Vec::new();
            for r in &rectangles {
                if r[0] <= x1 && r[2] >= x2 {
                    y_intervals.push((r[1], r[3]));
                }
            }

            if y_intervals.is_empty() {
                continue;
            }

            // Sort Y intervals by their starting boundary
            y_intervals.sort_unstable_by_key(|&val| val.0);

            // 4. Merge overlapping 1D Y-intervals
            let mut total_height: i64 = 0;
            let mut current_start = y_intervals[0].0;
            let mut current_end = y_intervals[0].1;

            for j in 1..y_intervals.len() {
                let next_start = y_intervals[j].0;
                let next_end = y_intervals[j].1;

                if next_start <= current_end {
                    // Overlapping or touching, extend the current interval
                    current_end = current_end.max(next_end);
                } else {
                    // Disjoint interval, add completed interval's height and reset
                    total_height += (current_end - current_start) as i64;
                    current_start = next_start;
                    current_end = next_end;
                }
            }
            // Add the final remaining interval segment
            total_height += (current_end - current_start) as i64;

            // 5. Accumulate strip area under modulo constraints
            let strip_area = (width * total_height) % r_mod;
            total_area = (total_area + strip_area) % r_mod;
        }

        total_area as i32
    }
}
