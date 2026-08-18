package main

import (
	"strconv"
)

func deserialize(s string) *NestedInteger {
	// Case 1: The input is just a single integer, not a list
	if s[0] != '[' {
		val, _ := strconv.Atoi(s)
		ni := &NestedInteger{}
		ni.SetInteger(val)
		return ni
	}

	var stack []*NestedInteger
	num := 0
	sign := 1
	inNum := false

	for i := 0; i < len(s); i++ {
		char := s[i]

		switch char {
		case '[':
			// Start of a new nested list
			stack = append(stack, &NestedInteger{})
		case '-':
			sign = -1
			inNum = true
		case ',', ']':
			// If we were parsing a number, finalize it and add to the current list
			if inNum {
				ni := NestedInteger{}
				ni.SetInteger(sign * num)
				stack[len(stack)-1].Add(ni)
				num = 0
				sign = 1
				inNum = false
			}

			// If it's a closing bracket, the current nested list level is complete
			if char == ']' {
				completedList := stack[len(stack)-1]
				stack = stack[:len(stack)-1] // Pop

				if len(stack) > 0 {
					// Add completed list to its parent
					stack[len(stack)-1].Add(*completedList)
				} else {
					// Outermost list is completely processed
					return completedList
				}
			}
		default:
			// Parse numeric digits
			num = num*10 + int(char-'0')
			inNum = true
		}
	}

	return nil
}
