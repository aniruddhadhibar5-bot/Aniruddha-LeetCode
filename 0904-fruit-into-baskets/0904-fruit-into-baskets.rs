use std::collections::HashMap;
use std::cmp::max;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut fruit_counts = HashMap::new();
        let mut max_fruits = 0;
        let mut left = 0;

        for right in 0..fruits.len() {
            // Include the fruit at the 'right' pointer into our baskets
            *fruit_counts.entry(fruits[right]).or_insert(0) += 1;

            // If we have more than 2 distinct types of fruits, shrink from the 'left'
            while fruit_counts.len() > 2 {
                let left_fruit = fruits[left];
                if let Some(count) = fruit_counts.get_mut(&left_fruit) {
                    *count -= 1;
                    if *count == 0 {
                        fruit_counts.remove(&left_fruit);
                    }
                }
                left += 1;
            }

            // Track the maximum window size
            max_fruits = max(max_fruits, right - left + 1);
        }

        max_fruits as i32
    }
}
