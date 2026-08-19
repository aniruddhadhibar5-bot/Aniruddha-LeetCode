impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        // Sort people's weights in ascending order
        people.sort_unstable();
        
        let mut boats = 0;
        let mut left = 0;
        let mut right = people.len() - 1;

        while left <= right {
            // If it's the last person remaining, they take a boat by themselves
            if left == right {
                boats += 1;
                break;
            }

            // If the heaviest and lightest can share a boat, include the lightest person
            if people[left] + people[right] <= limit {
                left += 1;
            }
            
            // The heaviest person always takes a boat in this turn
            right -= 1;
            boats += 1;
        }

        boats
    }
}
