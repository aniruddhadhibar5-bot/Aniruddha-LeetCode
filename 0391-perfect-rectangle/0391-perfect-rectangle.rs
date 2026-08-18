use std::collections::HashSet;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        
        let mut total_area: i64 = 0;
        let mut set = HashSet::new();

        for rect in rectangles {
            let x1 = rect[0];
            let y1 = rect[1];
            let x2 = rect[2];
            let y2 = rect[3];

            // Update bounding coordinates
            min_x = min_x.min(x1);
            min_y = min_y.min(y1);
            max_x = max_x.max(x2);
            max_y = max_y.max(y2);

            // Add to total area (cast to i64 to avoid integer overflow)
            total_area += (x2 - x1) as i64 * (y2 - y1) as i64;

            // Define the 4 corners of the current sub-rectangle
            let corners = [
                (x1, y1),
                (x1, y2),
                (x2, y1),
                (x2, y2)
            ];

            // Apply the cancel-out strategy for vertices
            for corner in corners {
                if !set.insert(corner) {
                    set.remove(&corner);
                }
            }
        }

        // 1. Check if the 4 expected outer bounding corners are the only ones left
        if set.len() != 4 
            || !set.contains(&(min_x, min_y)) 
            || !set.contains(&(min_x, max_y)) 
            || !set.contains(&(max_x, min_y)) 
            || !set.contains(&(max_x, max_y)) 
        {
            return false;
        }

        // 2. Check if the combined calculated area matches the actual bounding box area
        let expected_area = (max_x - min_x) as i64 * (max_y - min_y) as i64;
        
        total_area == expected_area
    }
}
