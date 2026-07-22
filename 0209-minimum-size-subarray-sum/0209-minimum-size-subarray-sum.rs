impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix = vec![0; n + 1];
        let mut min_len = usize::MAX;

        // Build prefix sums
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i];
        }

        // For each starting point, binary search the target window ending point
        for i in 0..n {
            let needed = prefix[i] + target;
            
            // Find the first index where prefix sum >= needed
            match prefix.binary_search(&needed) {
                Ok(idx) => {
                    min_len = min_len.min(idx - i);
                }
                Err(idx) => {
                    if idx <= n {
                        min_len = min_len.min(idx - i);
                    }
                }
            }
        }

        if min_len == usize::MAX {
            0
        } else {
            min_len as i32
        }
    }
}
