package main

func firstUniqChar(s string) int {
	// Fixed-size frequency array for 26 lowercase English letters
	var count [26]int

	// First pass: count the frequency of each character
	for i := 0; i < len(s); i++ {
		count[s[i]-'a']++
	}

	// Second pass: find the first character with a frequency of 1
	for i := 0; i < len(s); i++ {
		if count[s[i]-'a'] == 1 {
			return i
		}
	}

	// If no unique character exists
	return -1
}
