use std::collections::HashSet;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        // Group suffixes by their initial character (26 lowercase English letters)
        let mut groups: Vec<HashSet<String>> = vec![HashSet::new(); 26];
        
        for idea in ideas {
            let idx = (idea.as_bytes()[0] - b'a') as usize;
            let suffix = idea[1..].to_string();
            groups[idx].insert(suffix);
        }
        
        let mut ans: i64 = 0;
        
        // Compare every pair of initial characters
        for i in 0..25 {
            for j in (i + 1)..26 {
                if groups[i].is_empty() || groups[j].is_empty() {
                    continue;
                }
                
                // Find the number of overlapping suffixes
                let mut intersection_size = 0;
                for suffix in &groups[i] {
                    if groups[j].contains(suffix) {
                        intersection_size += 1;
                    }
                }
                
                // Suffixes unique to group i and group j
                let unique_i = (groups[i].len() - intersection_size) as i64;
                let unique_j = (groups[j].len() - intersection_size) as i64;
                
                // Each pair can form 2 valid company names (A B and B A)
                ans += unique_i * unique_j * 2;
            }
        }
        
        ans
    }
}
