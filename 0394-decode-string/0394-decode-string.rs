impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<(String, i32)> = Vec::new();
        let mut current_str = String::new();
        let mut current_num = 0;

        for ch in s.chars() {
            if ch.is_ascii_digit() {
                // Parse the repeating multiplier count k
                current_num = current_num * 10 + (ch as i32 - '0' as i32);
            } else if ch == '[' {
                // Save context on the stack and reset trackers
                stack.push((current_str, current_num));
                current_str = String::new();
                current_num = 0;
            } else if ch == ']' {
                // Retrieve the saved outer context
                if let Some((prev_str, repeat_count)) = stack.pop() {
                    let mut repeated = String::new();
                    for _ in 0..repeat_count {
                        repeated.push_str(&current_str);
                    }
                    // Combine the outer previous string with the repeated inner string
                    current_str = format!("{}{}", prev_str, repeated);
                }
            } else {
                // Accumulate lowercase English characters
                current_str.push(ch);
            }
        }

        current_str
    }
}
