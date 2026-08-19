impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        // Collect characters into a vector for easy mutability and indexing
        let mut chars: Vec<char> = dominoes.chars().collect();
        let n = chars.len();
        
        // Track the index of the left boundary force and the force character itself
        let mut left_idx = None;
        let mut left_force = 'L'; // Imaginary 'L' before index 0
        
        for i in 0..=n {
            // Use an imaginary 'R' at index n to close the final segment
            let current_force = if i == n { 'R' } else { chars[i] };
            
            if current_force == '.' {
                continue;
            }
            
            // Determine the start index of our segment of '.'
            let start = match left_idx {
                None => 0,
                Some(idx) => idx + 1,
            };
            let end = i;
            
            // Apply the forces based on the 4 boundary combinations
            if left_force == current_force {
                // Case 1 & 2: L...L or R...R -> fill the whole segment
                for j in start..end {
                    chars[j] = current_force;
                }
            } else if left_force == 'R' && current_force == 'L' {
                // Case 4: R...L -> forces meet in the middle
                let mut l = start;
                let mut r = end - 1;
                while l < r {
                    chars[l] = 'R';
                    chars[r] = 'L';
                    l += 1;
                    r -= 1;
                }
            }
            // Case 3: L...R -> stays '.' (no actions needed)
            
            // Update boundaries for the next segment
            left_idx = Some(i);
            left_force = current_force;
        }
        
        chars.iter().collect()
    }
}
