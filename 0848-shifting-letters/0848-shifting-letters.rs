impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut u8_chars: Vec<u8> = s.into_bytes();
        let n = u8_chars.len();
        let mut total_shift: i64 = 0;

        // Iterate backwards to compute suffix sums of shifts dynamically
        for i in (0..n).rev() {
            // Keep the accumulation within bounds of 26
            total_shift = (total_shift + shifts[i] as i64) % 26;
            
            // Calculate the new character byte value
            let original_offset = (u8_chars[i] - b'a') as i64;
            let new_offset = (original_offset + total_shift) % 26;
            
            u8_chars[i] = b'a' + new_offset as u8;
        }

        // Convert the modified byte vector back into a String safely
        unsafe { String::from_utf8_unchecked(u8_chars) }
    }
}
