use std::collections::HashMap;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut memo = HashMap::new();
        Self::dfs(&s1, &s2, &mut memo)
    }

    fn dfs<'a>(s1: &'a str, s2: &'a str, memo: &mut HashMap<(&'a str, &'a str), bool>) -> bool {
        // Base case 1: exact match
        if s1 == s2 {
            return true;
        }

        // Base case 2: lengths must match, and characters must be anagrams
        if s1.len() != s2.len() || !Self::is_anagram(s1, s2) {
            return false;
        }

        // Check if state is already computed
        if let Some(&res) = memo.get(&(s1, s2)) {
            return res;
        }

        let n = s1.len();
        
        // Try all possible split points
        for i in 1..n {
            // Case 1: No swap at this level
            // Compare s1[0..i] with s2[0..i] and s1[i..n] with s2[i..n]
            if Self::dfs(&s1[..i], &s2[..i], memo) && Self::dfs(&s1[i..], &s2[i..], memo) {
                memo.insert((s1, s2), true);
                return true;
            }

            // Case 2: Swapped at this level
            // Compare s1[0..i] with s2[n-i..n] and s1[i..n] with s2[0..n-i]
            if Self::dfs(&s1[..i], &s2[(n - i)..], memo) && Self::dfs(&s1[i..], &s2[..(n - i)], memo) {
                memo.insert((s1, s2), true);
                return true;
            }
        }

        memo.insert((s1, s2), false);
        false
    }

    // Helper to check if two strings have identical character counts (Pruning)
    fn is_anagram(s1: &str, s2: &str) -> bool {
        let mut counts = [0; 26];
        for b in s1.bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        for b in s2.bytes() {
            counts[(b - b'a') as usize] -= 1;
        }
        counts.iter().all(|&c| c == 0)
    }
}
