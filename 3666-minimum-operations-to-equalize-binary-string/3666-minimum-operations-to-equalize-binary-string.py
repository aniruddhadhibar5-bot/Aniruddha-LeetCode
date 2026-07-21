from collections import deque

class Solution(object):
    def minOperations(self, s, k):
        n = len(s)
        initial_zeros = s.count('0')
        
        # If the string already contains all '1's, 0 operations are required
        if initial_zeros == 0:
            return 0
            
        # Pointers to skip already visited states of the same parity
        next_even = list(range(n + 2))
        next_odd = list(range(n + 2))
        
        # Iterative path compression to find the next unvisited number
        def find_next(i, next_arr):
            curr = i
            path = []
            while curr < len(next_arr) and next_arr[curr] != curr:
                path.append(curr)
                curr = next_arr[curr]
            for node in path:
                next_arr[node] = curr
            return curr

        # Initialize the BFS queue with (current_zeros, operations_count)
        queue = deque([(initial_zeros, 0)])
        
        # Mark the initial state as visited by routing it to the next value
        if initial_zeros % 2 == 0:
            next_even[initial_zeros] = initial_zeros + 2
        else:
            next_odd[initial_zeros] = initial_zeros + 2
            
        while queue:
            cur, dist = queue.popleft()
            
            # Calculate the boundaries of valid zeros to flip
            x_min = max(0, k - n + cur)
            x_max = min(cur, k)
            
            # Determine the range of next possible zero counts
            nxt_min = cur + k - 2 * x_max
            nxt_max = cur + k - 2 * x_min
            
            # Select the parity tracking array
            parity = (cur + k) % 2
            next_arr = next_even if parity == 0 else next_odd
            
            # Find and visit all unvisited states within the boundary range
            i = find_next(nxt_min, next_arr)
            while i <= nxt_max:
                if i == 0:
                    return dist + 1
                    
                queue.append((i, dist + 1))
                # Remove 'i' from future consideration by linking it to i + 2
                next_arr[i] = i + 2
                i = find_next(i, next_arr)
                
        return -1
