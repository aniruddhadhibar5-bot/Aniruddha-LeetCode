use std::collections::BTreeMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let n = hand.len();
        let group_size = group_size as usize;
        
        // Quick check: total cards must be divisible by group_size
        if n % group_size != 0 {
            return false;
        }

        // Count frequencies of each card using a sorted map
        let mut counts = BTreeMap::new();
        for &card in &hand {
            *counts.entry(card).or_insert(0) += 1;
        }

        // Process cards starting from the smallest available value
        while let Some((&start_card, &count)) = counts.iter().next() {
            if count == 0 {
                counts.remove(&start_card);
                continue;
            }

            // We need to form 'count' number of groups starting with 'start_card'
            for i in 0..group_size {
                let current_card = start_card + i as i32;
                
                // Check if the required consecutive card exists with enough frequency
                if let Some(current_count) = counts.get_mut(&current_card) {
                    if *current_count < count {
                        return false;
                    }
                    *current_count -= count;
                } else {
                    return false; // Card is missing entirely
                }
            }
        }

        true
    }
}
