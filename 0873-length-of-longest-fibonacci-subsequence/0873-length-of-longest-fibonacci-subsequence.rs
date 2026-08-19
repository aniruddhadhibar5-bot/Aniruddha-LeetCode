use std::collections::HashMap;
use std::cmp::max;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        // Map to quickly find the index of a value in the array
        let mut index_map = HashMap::with_capacity(n);
        for (idx, &val) in arr.iter().enumerate() {
            index_map.insert(val, idx);
        }

        // dp maps a combined index key (i * n + j) to the length of the sequence ending at (i, j)
        let mut dp = HashMap::new();
        let mut max_len = 0;

        // Iterate through all pairs (i, j) where i < j
        for j in 0..n {
            for i in 0..j {
                let target = arr[j] - arr[i];
                
                // The preceding element must be smaller than arr[i]
                if target >= arr[i] {
                    continue;
                }

                // Look up if the preceding element exists in the array
                if let Some(&k) = index_map.get(&target) {
                    // Unique 1D encoding key for the pairs
                    let prev_key = k * n + i;
                    let curr_key = i * n + j;

                    // If a sequence already ended at (k, i), extend it. Otherwise, start a new sequence of length 3.
                    let len = *dp.get(&prev_key).unwrap_or(&2) + 1;
                    dp.insert(curr_key, len);

                    max_len = max(max_len, len);
                }
            }
        }

        if max_len >= 3 { max_len } else { 0 }
    }
}
