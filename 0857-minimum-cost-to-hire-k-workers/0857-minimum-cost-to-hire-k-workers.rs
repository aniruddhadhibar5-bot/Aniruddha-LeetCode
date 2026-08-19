use std::collections::BinaryHeap;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let n = quality.len();
        let k = k as usize;
        
        // Combine into a tuple of (ratio, quality)
        let mut workers: Vec<(f64, i32)> = Vec::with_capacity(n);
        for i in 0..n {
            let ratio = wage[i] as f64 / quality[i] as f64;
            workers.push((ratio, quality[i]));
        }

        // Sort workers by their wage-to-quality ratio in ascending order
        workers.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Max-Heap to track the smallest qualities seen so far
        let mut max_heap = BinaryHeap::new();
        let mut total_quality = 0;
        let mut min_cost = f64::MAX;

        for (ratio, q) in workers {
            total_quality += q;
            max_heap.push(q);

            // If we have more than k workers, evict the one with the highest quality
            if max_heap.len() > k {
                if let Some(highest_q) = max_heap.pop() {
                    total_quality -= highest_q;
                }
            }

            // Once we have exactly k workers, calculate the cost
            if max_heap.len() == k {
                let current_cost = ratio * total_quality as f64;
                if current_cost < min_cost {
                    min_cost = current_cost;
                }
            }
        }

        min_cost
    }
}
