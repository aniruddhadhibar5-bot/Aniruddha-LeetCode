impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        // Helper function to calculate Greatest Common Divisor (GCD)
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        }

        let g = gcd(p, q);
        
        // Simplify p and q to find the minimal room extensions
        let m = p / g; // horizontal extensions
        let n = q / g; // vertical extensions

        if m % 2 == 0 && n % 2 != 0 {
            2
        } else if m % 2 != 0 && n % 2 != 0 {
            1
        } else {
            0
        }
    }
}
