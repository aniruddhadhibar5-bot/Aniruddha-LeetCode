impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let pattern_bytes = pattern.as_bytes();
        let mut result = Vec::new();

        for word in words {
            if Self::is_match(word.as_bytes(), pattern_bytes) {
                result.push(word);
            }
        }

        result
    }

    fn is_match(word: &[u8], pattern: &[u8]) -> bool {
        // Arrays initialized to 0 to store mapping states (0 means unmapped)
        let mut p_to_w = [0u8; 26];
        let mut w_to_p = [0u8; 26];

        for i in 0..pattern.len() {
            let p_idx = (pattern[i] - b'a') as usize;
            let w_idx = (word[i] - b'a') as usize;

            // Check pattern-to-word mapping consistency
            if p_to_w[p_idx] == 0 {
                p_to_w[p_idx] = word[i];
            } else if p_to_w[p_idx] != word[i] {
                return false;
            }

            // Check word-to-pattern mapping consistency
            if w_to_p[w_idx] == 0 {
                w_to_p[w_idx] = pattern[i];
            } else if w_to_p[w_idx] != pattern[i] {
                return false;
            }
        }

        true
    }
}
