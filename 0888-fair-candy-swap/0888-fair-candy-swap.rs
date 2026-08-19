use std::collections::HashSet;

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let sum_a: i32 = alice_sizes.iter().sum();
        let sum_b: i32 = bob_sizes.iter().sum();
        
        // Calculate the required target difference delta = (sum_b - sum_a) / 2
        let delta = (sum_b - sum_a) / 2;
        
        // Store Bob's sizes in a HashSet for O(1) average lookup time
        let mut bob_set = HashSet::with_capacity(bob_sizes.len());
        for &y in &bob_sizes {
            bob_set.insert(y);
        }
        
        // Find the matching pair
        for &x in &alice_sizes {
            let target_y = x + delta;
            if bob_set.contains(&target_y) {
                return vec![x, target_y];
            }
        }
        
        vec![]
    }
}
