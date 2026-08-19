use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut word_counts = HashMap::new();
        
        for word in s1.split_whitespace() {
            *word_counts.entry(word).or_insert(0) += 1;
        }
        
        for word in s2.split_whitespace() {
            *word_counts.entry(word).or_insert(0) += 1;
        }
        
        word_counts.into_iter()
            .filter(|&(_, count)| count == 1)
            .map(|(word, _)| word.to_string())
            .collect()
    }
}
