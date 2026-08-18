package main

func lexicalOrder(n int) []int {
	result := make([]int, n)
	curr := 1

	for i := 0; i < n; i++ {
		result[i] = curr

		// 1. Try to go deeper down the trie branch (e.g., 1 -> 10 -> 100)
		if curr*10 <= n {
			curr *= 10
		} else {
			// 2. If going deeper exceeds n or we reach a digit ending in 9,
			// backtrack upward by stripping trailing digits, then move to the next sibling.
			for curr%10 == 9 || curr >= n {
				curr /= 10
			}
			curr++
		}
	}

	return result
}
