use std::collections::VecDeque;
use std::cmp::min;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;
        
        // P[i] stores the prefix sum of the first i elements
        let mut p = vec![0i64; n + 1];
        for i in 0..n {
            p[i + 1] = p[i] + nums[i] as i64;
        }

        let mut deque = VecDeque::new();
        let mut min_len = usize::MAX;

        for i in 0..=n {
            // 1. Shrink window from the left if the condition is satisfied
            while let Some(&front_idx) = deque.front() {
                if p[i] - p[front_idx] >= k {
                    min_len = min(min_len, i - front_idx);
                    deque.pop_front();
                } else {
                    break;
                }
            }

            // 2. Maintain increasing order of prefix values from the right
            while let Some(&back_idx) = deque.back() {
                if p[i] <= p[back_idx] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            // Push current index onto the monotonic queue
            deque.push_back(i);
        }

        if min_len == usize::MAX {
            -1
        } else {
            min_len as i32
        }
    }
}
