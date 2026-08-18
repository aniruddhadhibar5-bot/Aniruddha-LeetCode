use std::collections::HashMap;
use rand::{thread_rng, Rng};

struct Solution {
    // Maps each unique number to a list of its occurrence indices
    index_map: HashMap<i32, Vec<usize>>,
}

impl Solution {
    // Preprocess the input array -> O(n) time
    fn new(nums: Vec<i32>) -> Self {
        let mut index_map = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            index_map.entry(num).or_insert(Vec::new()).push(idx);
        }
        Solution { index_map }
    }
    
    // Pick an index with uniform random probability -> O(1) time
    fn pick(&self, target: i32) -> i32 {
        if let Some(indices) = self.index_map.get(&target) {
            let mut rng = thread_rng();
            let random_pos = rng.gen_range(0..indices.len());
            return indices[random_pos] as i32;
        }
        -1 // Fallback (problem statement guarantees target always exists)
    }
}
