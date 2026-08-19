
impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let mut result = Vec::new();
        let bytes = num.as_bytes();
        
        if Self::backtrack(0, bytes, &mut result) {
            result
        } else {
            Vec::new()
        }
    }

    fn backtrack(index: usize, bytes: &[u8], result: &mut Vec<i32>) -> bool {
        // Base case: If we have reached the end of the string and have at least 3 numbers
        if index == bytes.len() && result.len() >= 3 {
            return true;
        }

        let mut current_val: i64 = 0;
        
        for i in index..bytes.len() {
            // Leading zeros constraint: if the number starts with '0', it can only be 0 itself
            if i > index && bytes[index] == b'0' {
                break;
            }

            // Form the current number digit by digit
            current_val = current_val * 10 + (bytes[i] - b'0') as i64;

            // 32-bit signed integer limit check
            if current_val > i32::MAX as i64 {
                break;
            }

            let len = result.len();
            
            // If we already have 2 or more numbers, check the Fibonacci condition
            if len >= 2 {
                let sum = result[len - 1] as i64 + result[len - 2] as i64;
                
                // If current_val is less than the required sum, continue expanding the current slice
                if current_val < sum {
                    continue;
                }
                // If current_val exceeds the required sum, no need to look further down this branch
                if current_val > sum {
                    break;
                }
            }

            // Choose: Add the valid candidate number to our path
            result.push(current_val as i32);

            // Explore: Recursively check if this choice leads to a valid sequence
            if Self::backtrack(i + 1, bytes, result) {
                return true;
            }

            // Unchoose: Backtrack
            result.pop();
        }

        false
    }
}
