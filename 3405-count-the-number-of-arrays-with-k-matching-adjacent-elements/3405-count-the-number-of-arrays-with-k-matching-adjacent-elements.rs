impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let n = n as i64;
        let m = m as i64;
        let k = k as i64;
        let MOD: i64 = 1_000_000_007;

        // Base/Edge Case: If k is out of bounds, no such arrays exist
        if k < 0 || k >= n {
            return 0;
        }

        // 1. Calculate nCr(n - 1, k) % MOD
        let n_minus_1 = n - 1;
        let combinations = Self::n_cr(n_minus_1, k, MOD);

        // 2. Calculate (m - 1)^(n - 1 - k) % MOD
        let power_m_minus_1 = Self::power(m - 1, n_minus_1 - k, MOD);

        // 3. Multiply everything together: combinations * m * (m - 1)^(n - 1 - k)
        let mut ans = (combinations * m) % MOD;
        ans = (ans * power_m_minus_1) % MOD;

        ans as i32
    }

    // Helper function to calculate modular power: (base^exp) % MOD
    fn power(mut base: i64, mut exp: i64, m: i64) -> i64 {
        let mut res = 1;
        base %= m;
        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * base) % m;
            }
            base = (base * base) % m;
            exp /= 2;
        }
        res
    }

    // Helper function to calculate Fermat's Modular Inverse: (num^-1) % MOD
    fn mod_inverse(num: i64, m: i64) -> i64 {
        Self::power(num, m - 2, m)
    }

    // Helper function to compute nCr % MOD efficiently
    fn n_cr(n: i64, r: i64, m: i64) -> i64 {
        if r < 0 || r > n {
            return 0;
        }
        if r == 0 || r == n {
            return 1;
        }
        
        // Optimise to compute the smaller side of the symmetry
        let r = r.min(n - r);
        let mut num = 1;
        let mut den = 1;

        for i in 0..r {
            num = (num * (n - i)) % m;
            den = (den * (i + 1)) % m;
        }

        (num * Self::mod_inverse(den, m)) % m
    }
}
