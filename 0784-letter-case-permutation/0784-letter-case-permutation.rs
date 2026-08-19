impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut result = Vec::new();
        // Convert the string into a vector of chars for easy index manipulation
        let mut chars: Vec<char> = s.chars().collect();
        
        // Helper function for backtracking
        fn backtrack(index: usize, chars: &mut Vec<char>, result: &mut Vec<String>) {
            // Base case: if we processed all characters, add the copy to results
            if index == chars.len() {
                result.push(chars.iter().collect());
                return;
            }
            
            let original_char = chars[index];
            
            if original_char.is_ascii_alphabetic() {
                // Choice 1: Try lowercase transformation
                chars[index] = original_char.to_ascii_lowercase();
                backtrack(index + 1, chars, result);
                
                // Choice 2: Try uppercase transformation
                chars[index] = original_char.to_ascii_uppercase();
                backtrack(index + 1, chars, result);
                
                // Backtrack to original state (good practice, though overwritten above)
                chars[index] = original_char;
            } else {
                // If it's a digit, we only have one path
                backtrack(index + 1, chars, result);
            }
        }
        
        backtrack(0, &mut chars, &mut result);
        result
    }
}
