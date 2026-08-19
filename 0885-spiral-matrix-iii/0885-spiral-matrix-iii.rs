impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let total_cells = (rows * cols) as usize;
        let mut result = Vec::with_capacity(total_cells);
        
        // Direction vectors: East, South, West, North
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut dir_idx = 0;
        
        let mut r = r_start;
        let mut c = c_start;
        let mut step_len = 1;
        
        // Always record the initial starting cell
        result.push(vec![r, c]);
        
        while result.len() < total_cells {
            // Perform movement for the current side length
            for _ in 0..step_len {
                r += dirs[dir_idx].0;
                c += dirs[dir_idx].1;
                
                // Add to result if the current step lands inside grid boundaries
                if r >= 0 && r < rows && c >= 0 && c < cols {
                    result.push(vec![r, c]);
                    if result.len() == total_cells {
                        return result;
                    }
                }
            }
            
            // Turn clockwise: East -> South -> West -> North
            // Increment the step length after finishing both South (1) and North (3) limbs
            if dir_idx == 1 || dir_idx == 3 {
                step_len += 1;
            }
            dir_idx = (dir_idx + 1) % 4;
        }
        
        result
    }
}
