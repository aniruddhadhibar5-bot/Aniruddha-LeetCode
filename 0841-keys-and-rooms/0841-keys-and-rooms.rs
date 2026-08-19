use std::collections::VecDeque;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        
        // Start at room 0
        visited[0] = true;
        queue.push_back(0);
        
        let mut visited_count = 1;
        
        while let Some(current_room) = queue.pop_front() {
            // Check all keys available in the current room
            for &key in &rooms[current_room] {
                let key_idx = key as usize;
                
                // If we haven't visited this room yet, unlock and enter it
                if !visited[key_idx] {
                    visited[key_idx] = true;
                    visited_count += 1;
                    queue.push_back(key_idx);
                }
            }
        }
        
        // Return true if the total number of visited rooms equals n
        visited_count == n
    }
}
