class Solution:
    def findOrder(self, numCourses, prerequisites):
        # Step 1: Build the graph
        adj_list = {i: [] for i in range(numCourses)}
        for dest, src in prerequisites:
            adj_list[src].append(dest)
            
        # State tracking: 0 = unvisited, 1 = visiting, 2 = visited
        state = [0] * numCourses
        order = []
        
        # Helper DFS function
        def dfs(node):
            if state[node] == 1:
                return False  # Cycle detected
            if state[node] == 2:
                return True   # Already processed
                
            state[node] = 1   # Mark as visiting
            
            for neighbor in adj_list[node]:
                if not dfs(neighbor):
                    return False
                    
            state[node] = 2   # Mark as fully visited
            order.append(node) # Post-order processing
            return True
            
        # Step 2: Run DFS for each course
        for i in range(numCourses):
            if state[i] == 0:
                if not dfs(i):
                    return [] # Return empty array if a cycle is found
                    
        # Since we appended nodes post-order, reverse the array to get the correct path
        return order[::-1]
