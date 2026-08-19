use std::collections::HashSet;

impl Solution {
    pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
        let mut unique_signatures = HashSet::with_capacity(words.len());

        for word in words {
            // Arrays to hold the frequencies of characters at even and odd positions
            let mut even_counts = [0u8; 26];
            let mut odd_counts = [0u8; 26];

            for (i, &byte) in word.as_bytes().iter().enumerate() {
                let idx = (byte - b'a') as usize;
                if i % 2 == 0 {
                    even_counts[idx] += 1;
                } else {
                    odd_counts[idx] += 1;
                }
            }

            // Create a unique composite key combining both frequency states
            let signature = (even_counts, odd_counts);
            unique_signatures.insert(signature);
        }

        unique_signatures.len() as i32
    }
}
