use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        // Max-heap to track the fuel capacities of stations we've passed by
        let mut max_heap = BinaryHeap::new();
        
        let mut current_fuel = start_fuel as i64;
        let mut current_position = 0i64;
        let mut stops = 0;
        let mut station_idx = 0;
        let num_stations = stations.len();

        while current_position + current_fuel < target as i64 {
            // If there are still stations ahead and we have enough fuel to reach the next one
            if station_idx < num_stations && current_fuel >= (stations[station_idx][0] - current_position as i32) as i64 {
                let next_station_pos = stations[station_idx][0] as i64;
                let next_station_fuel = stations[station_idx][1];
                
                // Move to the next station, expending fuel for the distance traveled
                current_fuel -= next_station_pos - current_position;
                current_position = next_station_pos;
                
                // Store this station's fuel capacity as a future option
                max_heap.push(next_station_fuel);
                station_idx += 1;
            } else {
                // If we can't reach the next milestone, retroactively refuel from the best past station
                if let Some(highest_fuel) = max_heap.pop() {
                    current_fuel += highest_fuel as i64;
                    stops += 1;
                } else {
                    // No past stations available to refuel from; target is unreachable
                    return -1;
                }
            }
        }

        stops
    }
}
