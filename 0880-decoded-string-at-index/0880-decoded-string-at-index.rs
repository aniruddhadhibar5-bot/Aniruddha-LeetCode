impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut k = k as i64;
        let mut size: i64 = 0;
        let bytes = s.as_bytes();

        // 1. Forward pass: compute the total size of the decoded tape
        for &b in bytes {
            if b.is_ascii_digit() {
                let d = (b - b'0') as i64;
                size *= d;
            } else {
                size += 1;
            }
        }

        // 2. Backward pass: decode backwards to find the k-th character
        for i in (0..bytes.len()).rev() {
            let b = bytes[i];
            k %= size;

            // If k hits 0 on a letter, it means it is the last character of the current tape
            if k == 0 && b.is_ascii_alphabetic() {
                return (b as char).to_string();
            }

            if b.is_ascii_digit() {
                let d = (b - b'0') as i64;
                size /= d;
            } else {
                size -= 1;
            }
        }

        String::new()
    }
}
