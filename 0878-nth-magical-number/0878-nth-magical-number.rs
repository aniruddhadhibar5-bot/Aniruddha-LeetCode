impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let r_mod = 1_000_000_007i64;
        let n = n as i64;
        let a = a as i64;
        let b = b as i64;

        // Helper function to calculate Greatest Common Divisor (GCD)
        fn gcd(mut x: i64, mut y: i64) -> i64 {
            while y != 0 {
                let temp = y;
                y = x % y;
                x = temp;
            }
            x
        }

        // Calculate the Least Common Multiple (LCM) of a and b
        let lcm = (a * b) / gcd(a, b);

        // Define binary search boundaries
        let mut low = a.min(b);
        let mut high = n * a.min(b);
        let mut ans = high;

        while low <= high {
            let mid = low + (high - low) / 2;

            // Inclusion-Exclusion Principle formula to count magical numbers <= mid
            let count = (mid / a) + (mid / b) - (mid / lcm);

            if count >= n {
                ans = mid;
                high = mid - 1; // Try to look for a smaller matching number
            } else {
                low = mid + 1;  // Too few magical numbers, shift space higher
            }
        }

        (ans % r_mod) as i32
    }
}
