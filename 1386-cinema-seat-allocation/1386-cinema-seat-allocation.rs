use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        // Map to store row_id -> bitmask of reserved seats
        let mut row_masks: HashMap<i32, i32> = HashMap::new();

        for seat in reserved_seats {
            let row = seat[0];
            let col = seat[1];
            // Only care about seats 2 to 9
            if col >= 2 && col <= 9 {
                let mask = row_masks.entry(row).or_insert(0);
                *mask |= 1 << col;
            }
        }

        // Define masks for the three valid groups of 4 adjacent seats
        let left_mask = (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5);
        let middle_mask = (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7);
        let right_mask = (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9);

        // Start by assuming all rows are completely empty (2 families per row)
        // We will subtract or calculate specifically for rows that have reservations
        let mut ans = (n - row_masks.len() as i32) * 2;

        for (&_row, &mask) in &row_masks {
            let mut groups_in_row = 0;
            let mut left_taken = false;
            let mut right_taken = false;

            // Check if left 4 seats are completely free
            if (mask & left_mask) == 0 {
                groups_in_row += 1;
                left_taken = true;
            }

            // Check if right 4 seats are completely free
            if (mask & right_mask) == 0 {
                groups_in_row += 1;
                right_taken = true;
            }

            // If neither left nor right could fit a group, check the center group
            if !left_taken && !right_taken && (mask & middle_mask) == 0 {
                groups_in_row += 1;
            }

            ans += groups_in_row;
        }

        ans
    }
}
