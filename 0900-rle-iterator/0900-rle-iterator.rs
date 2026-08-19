pub struct RLEIterator {
    encoding: Vec<i32>,
    idx: usize,
}

impl RLEIterator {

    pub fn new(encoding: Vec<i32>) -> Self {
        RLEIterator {
            encoding,
            idx: 0,
        }
    }
    
    pub fn next(&mut self, mut n: i32) -> i32 {
        let len = self.encoding.len();

        while self.idx < len {
            let available_count = self.encoding[self.idx];

            if n <= available_count {
                // We can fully satisfy 'n' from the current sequence run block
                self.encoding[self.idx] -= n;
                return self.encoding[self.idx + 1];
            } else {
                // Current block is not enough; consume it completely and look further
                n -= available_count;
                self.encoding[self.idx] = 0;
                self.idx += 2;
            }
        }

        // If the loop terminates, the sequence has been completely exhausted
        -1
    }
}
