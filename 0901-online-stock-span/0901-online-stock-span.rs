pub struct StockSpanner {
    // Stack stores pairs of (price, span)
    stack: Vec<(i32, i32)>,
}

impl StockSpanner {

    pub fn new() -> Self {
        StockSpanner {
            stack: Vec::new(),
        }
    }
    
    pub fn next(&mut self, price: i32) -> i32 {
        let mut current_span = 1;

        // Pop elements from the stack while they are less than or equal to the current price
        while let Some(&(top_price, top_span)) = self.stack.last() {
            if top_price <= price {
                current_span += top_span;
                self.stack.pop();
            } else {
                break;
            }
        }

        // Push the consolidated (price, span) pair onto the monotonic stack
        self.stack.push((price, current_span));
        
        current_span
    }
}
