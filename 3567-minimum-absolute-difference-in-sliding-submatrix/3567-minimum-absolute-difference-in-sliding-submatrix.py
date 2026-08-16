class Solution:
    def minAbsDiff(self, grid, k):
        # 1. Get matrix dimensions safely
        m = len(grid)
        if m == 0:
            return []
        n = len(grid[0])
        
        # 2. Pre-allocate the result matrix with 0s correctly
        res = [[0] * (n - k + 1) for _ in range(m - k + 1)]
        
        # 3. Slide the k x k window across the grid
        for i in range(m - k + 1):
            for j in range(n - k + 1):
                kgrid = []
                
                # Gather all elements within the current submatrix
                for x in range(i, i + k):
                    for y in range(j, j + k):
                        kgrid.append(grid[x][y])
                
                # Sort elements to find adjacent differences
                kgrid.sort()
                
                kmin = float('inf')
                # Walk through adjacent pairs to find the minimum absolute difference
                for t in range(1, len(kgrid)):
                    if kgrid[t] == kgrid[t - 1]:
                        continue
                    diff = kgrid[t] - kgrid[t - 1]
                    if diff < kmin:
                        kmin = diff
                
                # If distinct numbers were found, store their minimum absolute difference
                if kmin != float('inf'):
                    res[i][j] = kmin
                    
        return res
