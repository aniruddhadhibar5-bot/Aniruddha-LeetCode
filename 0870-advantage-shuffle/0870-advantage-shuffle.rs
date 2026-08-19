use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let n = nums1.len();
        let mut counts = BTreeMap::new();
        
        // Count frequencies of each number in nums1
        for num in nums1 {
            *counts.entry(num).or_insert(0) += 1;
        }
        
        let mut result = vec![0; n];
        
        for i in 0..n {
            let target = nums2[i];
            
            // Look for the smallest value strictly greater than target
            // We search from (target + 1) onwards
            let mut choice = None;
            if let Some((&val, _)) = counts.range((Included(target + 1), Unbounded)).next() {
                choice = Some(val);
            }
            
            if let Some(val) = choice {
                result[i] = val;
                let count = counts.get_mut(&val).unwrap();
                *count -= 1;
                if *count == 0 {
                    counts.remove(&val);
                }
            } else {
                // If we can't beat it, grab the absolute smallest element remaining
                let (&smallest, count) = counts.iter_mut().next().unwrap();
                result[i] = smallest;
                *count -= 1;
                if *count == 0 {
                    let k = smallest;
                    counts.remove(&k);
                }
            }
        }
        
        result
    }
}
