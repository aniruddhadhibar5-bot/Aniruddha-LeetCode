impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut ans = 0;
        let mut ones = 0;
        let bytes = s.as_bytes();

        for i in 0..bytes.len() {
            if bytes[i] == b'1' {
                ones += 1;
            } else if i > 0 && bytes[i - 1] == b'1' {
                // We encountered the start of a '0' block.
                // All '1's accumulated so far will jump over this block.
                ans += ones;
            }
        }

        ans
    }
}
