package main

func minChanges(nums []int, k int) int {
	n := len(nums)
	// Difference array to efficiently perform range updates across [0, k]
	diff := make([]int, k+2)

	for i := 0; i < n/2; i++ {
		a := nums[i]
		b := nums[n-1-i]
		if a > b {
			a, b = b, a
		}

		// Current absolute difference (takes 0 changes initially)
		d := b - a

		// Maximum absolute difference achievable by changing exactly 1 element
		m := b
		if k-a > m {
			m = k - a
		}

		// Target differences in range [0, m] require 1 change
		diff[0] += 1 // FIXED: Explicitly target index 0
		diff[m+1] -= 1

		// Target differences in range [m+1, k] require 2 changes
		diff[m+1] += 2
		diff[k+1] -= 2

		// Since target 'd' was already counted as 1 change above,
		// subtract 1 to set its actual net cost to 0 changes.
		diff[d] -= 1
		diff[d+1] += 1
	}

	// Process prefix sums to find the target absolute difference X with minimum cost
	minChanges := n
	currentChanges := 0

	for x := 0; x <= k; x++ {
		currentChanges += diff[x]
		if currentChanges < minChanges {
			minChanges = currentChanges
		}
	}

	return minChanges
}
