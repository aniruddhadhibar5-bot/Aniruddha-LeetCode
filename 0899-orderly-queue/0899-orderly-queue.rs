impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k > 1 {
            // Case 1: If k > 1, we can bubble sort the entire string
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            chars.iter().collect()
        } else {
            // Case 2: If k == 1, we can only cyclically rotate the string
            let mut smallest = s.clone();
            let mut current = s;

            for _ in 1..smallest.len() {
                // Remove the first character and append it to the end
                let first_char = current.remove(0);
                current.push(first_char);
                
                if current < smallest {
                    smallest = current.clone();
                }
            }

            smallest
        }
    }
}
