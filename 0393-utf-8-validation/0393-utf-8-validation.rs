impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut expected_continuation_bytes = 0;

        for num in data {
            // Only look at the least significant 8 bits
            let byte = (num & 255) as u8;

            if expected_continuation_bytes == 0 {
                // Determine how many bytes this UTF-8 character contains
                if (byte >> 7) == 0b0 {
                    expected_continuation_bytes = 0; // 1-byte character
                } else if (byte >> 5) == 0b110 {
                    expected_continuation_bytes = 1; // 2-byte character
                } else if (byte >> 4) == 0b1110 {
                    expected_continuation_bytes = 2; // 3-byte character
                } else if (byte >> 3) == 0b11110 {
                    expected_continuation_bytes = 3; // 4-byte character
                } else {
                    return false; // Invalid leading byte sequence
                }
            } else {
                // Check if this byte is a valid continuation byte (must start with '10')
                if (byte >> 6) != 0b10 {
                    return false;
                }
                expected_continuation_bytes -= 1;
            }
        }

        // If we processed everything perfectly, expected_continuation_bytes should be 0
        expected_continuation_bytes == 0
    }
}
