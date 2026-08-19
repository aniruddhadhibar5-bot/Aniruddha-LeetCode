impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let n_str = n.to_string();
        let n_chars: Vec<char> = n_str.chars().collect();
        let len_n = n_chars.len();
        let num_digits = digits.len();

        let mut total_count = 0;

        // Part 1: Count all valid numbers with strictly fewer digits than n
        for i in 1..len_n {
            total_count += (num_digits as i32).pow(i as u32);
        }

        // Part 2: Count valid numbers with exactly the same number of digits as n
        for i in 0..len_n {
            let mut prefix_matched = false;

            for d_str in &digits {
                let d_char = d_str.chars().next().unwrap();

                if d_char < n_chars[i] {
                    // If the digit is strictly smaller, all remaining spots can take any digit
                    let remaining_slots = len_n - 1 - i;
                    total_count += (num_digits as i32).pow(remaining_slots as u32);
                } else if d_char == n_chars[i] {
                    // If it matches exactly, we can continue checking the next position
                    prefix_matched = true;
                    break;
                } else {
                    // Since 'digits' is sorted, any subsequent digit will be larger
                    break;
                }
            }

            // If the current prefix character of n could not be matched, we cannot proceed
            if !prefix_matched {
                return total_count;
            }
        }

        // If we successfully finished the loop, it means n itself can be formed from the set
        total_count + 1
    }
}
