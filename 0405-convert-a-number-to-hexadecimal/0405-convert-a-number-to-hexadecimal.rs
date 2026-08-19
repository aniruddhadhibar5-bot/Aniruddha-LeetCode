impl Solution {
    pub fn to_hex(num: i32) -> String {
        // Base case
        if num == 0 {
            return "0".to_string();
        }

        // Cast to u32 to handle negative numbers via two's complement bit layout
        let mut val = num as u32;
        let hex_chars = b"0123456789abcdef";
        let mut result = Vec::new();

        // Process up to 8 blocks of 4-bits (since a 32-bit integer has exactly 8 hex digits)
        while val > 0 {
            // Isolate the lowest 4 bits
            let rem = (val & 15) as usize;
            result.push(hex_chars[rem] as char);
            
            // Shift right by 4 bits to process the next digit
            val >>= 4;
        }

        // Since we processed digits from right to left, reverse the result string
        result.into_iter().rev().collect()
    }
}
