impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut head = 1;
        let mut step = 1;
        let mut remaining = n;
        let mut left_to_right = true;

        while remaining > 1 {
            // If we move left to right, or if we move right to left and the count is odd,
            // the head element will be removed, moving it forward by the current step size.
            if left_to_right || remaining % 2 == 1 {
                head += step;
            }

            // Shrink the pool size by half
            remaining /= 2;
            // Double the distance step between remaining elements
            step *= 2;
            // Flip the scanning direction
            left_to_right = !left_to_right;
        }

        head
    }
}
