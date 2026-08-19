impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        // Since constraints state 0 <= answers[i] < 1000, 
        // we can use a fixed-size array of 1000 elements.
        let mut counts = [0; 1000];
        
        // Count the frequency of each answer
        for ans in answers {
            counts[ans as usize] += 1;
        }
        
        let mut total_rabbits = 0;
        
        // Calculate the minimum rabbits needed
        for (x, &count) in counts.iter().enumerate() {
            if count == 0 {
                continue;
            }
            
            let group_size = x + 1;
            // Ceiling division logic using integer math: (count + x) / group_size
            let groups = (count + x) / group_size;
            total_rabbits += groups * group_size;
        }
        
        total_rabbits as i32
    }
}
