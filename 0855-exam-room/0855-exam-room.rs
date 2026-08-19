use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Interval {
    start: i32,
    end: i32,
    n: i32,
}

impl Interval {
    fn new(start: i32, end: i32, n: i32) -> Self {
        Interval { start, end, n }
    }

    // Calculates the maximum distance to the closest person inside this interval
    fn dist(&self) -> i32 {
        if self.start == -1 {
            self.end
        } else if self.end == self.n {
            self.n - 1 - self.start
        } else {
            (self.end - self.start) / 2
        }
    }

    // Determines the exact seat position chosen inside this interval
    fn seat(&self) -> i32 {
        if self.start == -1 {
            0
        } else if self.end == self.n {
            self.n - 1
        } else {
            self.start + (self.end - self.start) / 2
        }
    }
}

// Custom sorting rules for our intervals to build a Max-Heap structure
impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        let d1 = self.dist();
        let d2 = other.dist();
        if d1 != d2 {
            // Priority 1: Maximize distance (descending order)
            d2.cmp(&d1)
        } else {
            // Priority 2: Tie-break with lowest seat number (ascending order)
            self.seat().cmp(&other.seat())
        }
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct ExamRoom {
    n: i32,
    intervals: BTreeSet<Interval>,
    occupied_seats: BTreeSet<i32>,
}

impl ExamRoom {
    pub fn new(n: i32) -> Self {
        let mut intervals = BTreeSet::new();
        // Initially, the entire row from -1 to n is one big available interval
        intervals.insert(Interval::new(-1, n, n));

        ExamRoom {
            n,
            intervals,
            occupied_seats: BTreeSet::new(),
        }
    }
    
    pub fn seat(&mut self) -> i32 {
        // Pop the optimal interval from the top of the sorted set
        let current_interval = *self.intervals.iter().next().unwrap();
        self.intervals.remove(&current_interval);

        let p = current_interval.seat();
        
        // Split the old interval into two new sub-intervals around the new seat 'p'
        self.intervals.insert(Interval::new(current_interval.start, p, self.n));
        self.intervals.insert(Interval::new(p, current_interval.end, self.n));

        self.occupied_seats.insert(p);
        p
    }
    
    pub fn leave(&mut self, p: i32) {
        // Find the immediate left and right neighbors of seat 'p'
        let pred = *self.occupied_seats.range(..p).next_back().unwrap_or(&-1);
        let succ = *self.occupied_seats.range((p + 1)..).next().unwrap_or(&self.n);

        // Remove the two old sub-intervals that were bounded by 'p'
        self.intervals.remove(&Interval::new(pred, p, self.n));
        self.intervals.remove(&Interval::new(p, succ, self.n));

        // Merge them back into a single continuous interval
        self.intervals.insert(Interval::new(pred, succ, self.n));
        self.occupied_seats.remove(&p);
    }
}
