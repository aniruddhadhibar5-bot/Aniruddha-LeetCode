package main

func minFlips(mat [][]int) int {
	m := len(mat)
	n := len(mat[0])

	// Convert the initial matrix state into a single integer bitmask
	startState := 0
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if mat[i][j] == 1 {
				pos := i*n + j
				startState |= (1 << pos)
			}
		}
	}

	// If it's already a zero matrix
	if startState == 0 {
		return 0
	}

	// Directions array for exploring neighbors (current, up, down, left, right)
	dirs := [][]int{{0, 0}, {-1, 0}, {1, 0}, {0, -1}, {0, 1}}

	// BFS setup
	queue := []int{startState}
	visited := make(map[int]bool)
	visited[startState] = true
	steps := 0

	for len(queue) > 0 {
		size := len(queue)
		for s := 0; s < size; s++ {
			curr := queue[0]
			queue = queue[1:]

			// Check all possible cells to flip
			for i := 0; i < m; i++ {
				for j := 0; j < n; j++ {
					nextState := curr

					// Flip the cell (i, j) and its valid neighbors
					for _, d := range dirs {
						ni, nj := i+d[0], j+d[1]
						if ni >= 0 && ni < m && nj >= 0 && nj < n {
							pos := ni*n + nj
							nextState ^= (1 << pos) // XOR to flip bit (0->1 or 1->0)
						}
					}

					// If we successfully reached the zero matrix state
					if nextState == 0 {
						return steps + 1
					}

					// If this state hasn't been visited yet, add to queue
					if !visited[nextState] {
						visited[nextState] = true
						queue = append(queue, nextState)
					}
				}
			}
		}
		steps++
	}

	return -1
}
