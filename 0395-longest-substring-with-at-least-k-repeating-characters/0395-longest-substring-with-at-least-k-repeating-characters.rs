impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::helper(s.as_bytes(), k as usize) as i32
    }

    fn helper(bytes: &[u8], k: usize) -> usize {
        if bytes.len() < k {
            return 0;
        }

        // Count frequencies of each character
        let mut counts = [0; 26];
        for &b in bytes {
            counts[(b - b'a') as usize] += 1;
        }

        // Search for any character that appears less than k times
        for (i, &b) in bytes.iter().enumerate() {
            if counts[(b - b'a') as usize] < k {
                // Split the string into two halves around this invalid character
                let left_res = Self::helper(&bytes[0..i], k);
                let right_res = Self::helper(&bytes[i + 1..], k);
                
                return left_res.max(right_res);
            }
        }

        // If no invalid characters were found, the entire substring is valid
        bytes.len()
    }
}
