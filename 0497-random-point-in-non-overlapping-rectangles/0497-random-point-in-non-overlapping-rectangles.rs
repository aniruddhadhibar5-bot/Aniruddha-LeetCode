use rand::{Rng, thread_rng};

struct Solution {
    rects: Vec<Vec<i32>>,
    prefix_sums: Vec<i32>,
    total_points: i32,
}

/** 
 * `&self` means the method is immutable and can be called multiple times.
 */
impl Solution {

    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut prefix_sums = Vec::with_capacity(rects.len());
        let mut sum = 0;
        
        for r in &rects {
            // Number of integer points in the current rectangle
            let pts = (r[2] - r[0] + 1) * (r[3] - r[1] + 1);
            sum += pts;
            prefix_sums.push(sum);
        }
        
        Solution {
            rects,
            prefix_sums,
            total_points: sum,
        }
    }
    
    fn pick(&self) -> Vec<i32> {
        let mut rng = thread_rng();
        // Pick a uniform random point index across all rectangles combined
        let target = rng.gen_range(0..self.total_points);
        
        // Use binary search to find which rectangle contains this point index
        let idx = self.prefix_sums.partition_point(|&sum| sum <= target);
        
        // Calculate the local index within the chosen rectangle
        let local_idx = if idx == 0 {
            target
        } else {
            target - self.prefix_sums[idx - 1]
        };
        
        let r = &self.rects[idx];
        let width = r[2] - r[0] + 1;
        
        // Map the linear 1D local index to 2D (x, y) coordinates
        let x = r[0] + (local_idx % width);
        let y = r[1] + (local_idx / width);
        
        vec![x, y]
    }
}
