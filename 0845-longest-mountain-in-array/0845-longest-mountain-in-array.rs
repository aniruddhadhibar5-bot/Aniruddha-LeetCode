use std::cmp::max;

impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut max_len = 0;
        let mut base = 0;

        while base < n {
            let mut peak = base;

            // 1. Move up the hill to find a peak
            if peak + 1 < n && arr[peak] < arr[peak + 1] {
                while peak + 1 < n && arr[peak] < arr[peak + 1] {
                    peak += 1;
                }

                // 2. We are at a peak. Now verify there is a valid downhill slope
                if peak + 1 < n && arr[peak] > arr[peak + 1] {
                    let mut end = peak;
                    while end + 1 < n && arr[end] > arr[end + 1] {
                        end += 1;
                    }

                    // 3. Valid mountain found! Record its length
                    max_len = max(max_len, (end - base + 1) as i32);
                    
                    // Move the base to the end of this mountain for the next search
                    base = end;
                    continue;
                }
            }

            // If it's not a valid mountain, just advance the base pointer
            base = max(base + 1, peak);
        }

        max_len
    }
}
