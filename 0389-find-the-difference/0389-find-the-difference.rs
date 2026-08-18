impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut res = 0u8;
        
        // XOR all bytes in string s
        for &byte in s.as_bytes() {
            res ^= byte;
        }
        
        // XOR all bytes in string t
        for &byte in t.as_bytes() {
            res ^= byte;
        }
        
        // The remaining byte is the added character
        res as char
    }
}
