impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        // If the string contains at least one vowel, Alice wins.
        s.bytes().any(|b| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u'))
    }
}
