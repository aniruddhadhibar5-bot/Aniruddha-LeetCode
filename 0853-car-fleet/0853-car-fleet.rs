impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n = position.len();
        if n == 0 {
            return 0;
        }

        // Combine position and speed into a single vector of pairs
        let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();

        // Sort cars by position in descending order (closest to target first)
        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut fleets = 0;
        let mut current_lead_time = 0.0;

        for (pos, spd) in cars {
            // Calculate time to target as a floating-point number
            let time = (target - pos) as f64 / spd as f64;

            // If this car takes more time than the current fleet leader ahead of it,
            // it cannot catch up. It starts a new fleet.
            if time > current_lead_time {
                fleets += 1;
                current_lead_time = time;
            }
        }

        fleets
    }
}
