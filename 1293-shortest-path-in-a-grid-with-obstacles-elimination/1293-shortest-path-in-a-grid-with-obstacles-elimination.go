package main

func shortestPath(grid [][]int, k int) int {
	m := len(grid)
	n := len(grid[0])

	// Optimization: If k is large enough to clear a direct Manhattan path,
	// we can bypass BFS completely and return the shortest geometric distance.
	if k >= m+n-2 {
		return m + n - 2
	}

	// 3D array to track visited states: visited[row][col] stores the max remaining k seen so far
	visited := make([][]int, m)
	for i := range visited {
		visited[i] = make([]int, n)
		for j := range visited[i] {
			visited[i][j] = -1 // Initialize with -1 to mean unvisited
		}
	}

	// State structure for our BFS queue
	type State struct {
		r, c, remK int
	}

	// Directions for moving Up, Down, Left, Right
	dirs := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

	queue := []State{{0, 0, k}}
	visited[0][0] = k
	steps := 0

	for len(queue) > 0 {
		size := len(queue)
		for s := 0; s < size; s++ {
			curr := queue[0]
			queue = queue[1:]

			// Reached destination
			if curr.r == m-1 && curr.c == n-1 {
				return steps
			}

			// Explore neighbors
			for _, d := range dirs {
				nr, nc := curr.r+d[0], curr.c+d[1]

				// Check matrix boundary limits
				if nr >= 0 && nr < m && nc >= 0 && nc < n {
					nextK := curr.remK - grid[nr][nc]

					// If we have enough k left and this path preserves MORE remaining k than seen before
					if nextK >= 0 && nextK > visited[nr][nc] {
						visited[nr][nc] = nextK
						queue = append(queue, State{nr, nc, nextK})
					}
				}
			}
		}
		steps++
	}

	return -1
}
