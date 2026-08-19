use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        // Quick validation: The first jump must always be exactly 1 unit.
        // Since stones[0] is 0, the second stone must be at position 1.
        if stones[1] != 1 {
            return false;
        }

        // Map stone positions to their respective array indices for O(1) lookups
        let mut stone_positions = HashMap::new();
        for (idx, &pos) in stones.iter().enumerate() {
            stone_positions.insert(pos, idx);
        }

        let mut memo = HashMap::new();
        
        // Start recursion from index 1 (the stone at position 1) with an initial jump of 1
        Self::dfs(1, 1, &stones, &stone_positions, &mut memo)
    }

    fn dfs(
        curr_idx: usize,
        last_jump: i32,
        stones: &Vec<i32>,
        stone_positions: &HashMap<i32, usize>,
        memo: &mut HashMap<(usize, i32), bool>,
    ) -> bool {
        // Base Case: If the frog has successfully landed on the final stone
        if curr_idx == stones.len() - 1 {
            return true;
        }

        // Return cached result if this state has already been evaluated
        if let Some(&ans) = memo.get(&(curr_idx, last_jump)) {
            return ans;
        }

        let curr_pos = stones[curr_idx];

        // The frog can choose a next jump step of k-1, k, or k+1
        for next_jump in (last_jump - 1)..=(last_jump + 1) {
            if next_jump <= 0 {
                continue;
            }

            let next_pos = curr_pos + next_jump;

            // Check if there is a stone at the landing position
            if let Some(&next_idx) = stone_positions.get(&next_pos) {
                if Self::dfs(next_idx, next_jump, stones, stone_positions, memo) {
                    memo.insert((curr_idx, last_jump), true);
                    return true;
                }
            }
        }

        memo.insert((curr_idx, last_jump), false);
        false
    }
}
