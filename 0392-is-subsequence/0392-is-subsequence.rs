impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        
        let mut s_idx = 0;
        let mut t_idx = 0;
        
        while s_idx < s_bytes.len() && t_idx < t_bytes.len() {
            if s_bytes[s_idx] == t_bytes[t_idx] {
                s_idx += 1;
            }
            t_idx += 1;
        }
        
        s_idx == s_bytes.len()
    }
}
