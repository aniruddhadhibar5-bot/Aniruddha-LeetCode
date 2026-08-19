impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = Vec::new();

        for ch in s.chars() {
            if ch == '(' {
                // 0 serves as a marker for a newly opened nesting layer
                stack.push(0);
            } else {
                let mut inner_score = 0;
                
                // Accumulate all computed scores inside the current layer
                while let Some(&top) = stack.last() {
                    if top == 0 {
                        break;
                    }
                    inner_score += stack.pop().unwrap();
                }
                
                // Pop the opening layer marker '0'
                stack.pop();
                
                // Compute the layer score: "()" becomes 1, "(A)" becomes 2 * A
                let layer_score = if inner_score == 0 { 1 } else { 2 * inner_score };
                stack.push(layer_score);
            }
        }

        // Sum up any remaining top-level independent components (e.g., AB -> A + B)
        stack.into_iter().sum()
    }
}
