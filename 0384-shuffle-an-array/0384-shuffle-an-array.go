package main

import (
	"math/rand"
)

type Solution struct {
	original []int
	current  []int
}

// Constructor initializes the object with the integer array nums.
func Constructor(nums []int) Solution {
	// Create deep copies to prevent side effects
	orig := make([]int, len(nums))
	curr := make([]int, len(nums))
	copy(orig, nums)
	copy(curr, nums)
	
	return Solution{
		original: orig,
		current:  curr,
	}
}

// Reset resets the array to its original configuration and returns it.
func (this *Solution) Reset() []int {
	copy(this.current, this.original)
	return this.current
}

// Shuffle returns a random shuffling of the array.
func (this *Solution) Shuffle() []int {
	n := len(this.current)
	
	// Fisher-Yates shuffle algorithm
	for i := n - 1; i > 0; i-- {
		// Pick a random index from 0 to i
		j := rand.Intn(i + 1)
		// Swap elements
		this.current[i], this.current[j] = this.current[j], this.current[i]
	}
	
	return this.current
}
