use std::collections::HashSet;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        // Strings must be of identical length
        if s.len() != goal.len() {
            return false;
        }

        let s_bytes = s.as_bytes();
        let goal_bytes = goal.as_bytes();

        if s == goal {
            // If identical, we need at least one duplicate character to swap with itself
            let mut unique_chars = HashSet::new();
            for &b in s_bytes {
                if !unique_chars.insert(b) {
                    return true; // Found a duplicate character
                }
            }
            return false;
        }

        // Find all indices where characters differ
        let mut diff_indices = Vec::new();
        for i in 0..s_bytes.len() {
            if s_bytes[i] != goal_bytes[i] {
                diff_indices.push(i);
                // Optimization: if there are more than 2 mismatches, a single swap can't fix it
                if diff_indices.len() > 2 {
                    return false;
                }
            }
        }

        // To fix with exactly 1 swap, there must be exactly 2 differences
        if diff_indices.len() != 2 {
            return false;
        }

        let first = diff_indices[0];
        let second = diff_indices[1];

        // Check if swapping the two positions matches the goal
        s_bytes[first] == goal_bytes[second] && s_bytes[second] == goal_bytes[first]
    }
}
