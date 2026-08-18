impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        let mut stack: Vec<char> = Vec::new();

        for ch in num.chars() {
            // While stack is not empty, k > 0, and current digit is smaller than the top of stack
            while k > 0 && !stack.is_empty() && *stack.last().unwrap() > ch {
                stack.pop();
                k -= 1;
            }
            stack.push(ch);
        }

        // If we still need to remove digits, remove them from the end
        while k > 0 && !stack.is_empty() {
            stack.pop();
            k -= 1;
        }

        // Build the result string while ignoring leading zeros
        let mut result = String::new();
        let mut leading_zero = true;

        for ch in stack {
            if leading_zero && ch == '0' {
                continue;
            }
            leading_zero = false;
            result.push(ch);
        }

        // If the resulting string is empty, the answer is "0"
        if result.is_empty() {
            "0".to_string()
        } else {
            result
        }
    }
}
