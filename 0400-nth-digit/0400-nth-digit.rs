impl Solution {
    pub fn find_nth_digit(mut n: i32) -> i32 {
        let mut len = 1;         // Digit length of numbers in the current group
        let mut count: i64 = 9;  // How many numbers exist in this group
        let mut start: i64 = 1;  // The starting number of this group

        // Step 1: Find the group category (1-digit, 2-digit, 3-digit, etc.)
        while (n as i64) > count * len as i64 {
            n -= (count * len as i64) as i32;
            len += 1;
            count *= 10;
            start *= 10;
        }

        // Step 2: Find the exact number where the nth digit resides
        // (n - 1) is used for 0-indexed offset adjustment
        let target_num = start + ((n - 1) / len) as i64;

        // Step 3: Find the exact digit within the target number
        let digit_idx = ((n - 1) % len) as usize;
        let num_str = target_num.to_string();
        
        num_str.as_bytes()[digit_idx] as i32 - '0' as i32
    }
}
