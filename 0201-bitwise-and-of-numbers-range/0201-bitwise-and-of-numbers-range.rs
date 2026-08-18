impl Solution {
    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        let mut shifts = 0;
        
        // Shift right until left and right match (finding the common prefix)
        while left < right {
            left >>= 1;
            right >>= 1;
            shifts += 1;
        }
        
        // Shift back left to restore the trailing zeros
        left << shifts
    }
}
