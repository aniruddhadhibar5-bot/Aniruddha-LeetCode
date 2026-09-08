impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        let mut total_commas = 0;

        // Numbers from 1,000 to 999,999 each contain exactly 1 comma
        if n >= 1000 {
            total_commas += (n.min(999_999) - 1000 + 1) * 1;
        }

        // Numbers from 1,000,000 to 999,999,999 each contain exactly 2 commas
        if n >= 1_000_000 {
            total_commas += (n.min(999_999_999) - 1_000_000 + 1) * 2;
        }

        total_commas
    }
}
