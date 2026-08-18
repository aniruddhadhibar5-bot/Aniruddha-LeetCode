impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = Self::get_next(n);

        // Move pointers until they either meet or the fast pointer hits 1
        while fast != 1 && slow != fast {
            slow = Self::get_next(slow);
            fast = Self::get_next(Self::get_next(fast));
        }

        fast == 1
    }

    // Helper function to calculate the sum of squares of the digits of a number
    fn get_next(mut num: i32) -> i32 {
        let mut total_sum = 0;
        while num > 0 {
            let digit = num % 10;
            total_sum += digit * digit;
            num /= 10;
        }
        total_sum
    }
}
