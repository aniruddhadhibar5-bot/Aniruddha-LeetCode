import sys
from collections import defaultdict

# Increase recursion depth limit to handle up to 10^5 nodes safely
sys.setrecursionlimit(200000)

class Solution:
    def criticalConnections(self, n, connections):
        # 1. Build the adjacency list graph
        graph = defaultdict(list)
        for u, v in connections:
            graph[u].append(v)
            graph[v].append(u)
            
        # 2. Track discovery time and lowest reachable node order
        discovery_time = [-1] * n
        low = [-1] * n
        bridges = []
        self.time = 0
        
        # 3. Traversal logic function
        def dfs(node, parent):
            discovery_time[node] = low[node] = self.time
            self.time += 1
            
            for neighbor in graph[node]:
                if neighbor == parent:
                    continue
                    
                if discovery_time[neighbor] == -1:
                    # Neighbor hasn't been visited yet
                    dfs(neighbor, node)
                    
                    # Update lowest reachable ID from child subtree
                    low[node] = min(low[node], low[neighbor])
                    
                    # If neighbor cannot reach node or its ancestors, it's a bridge
                    if low[neighbor] > discovery_time[node]:
                        bridges.append([node, neighbor])
                else:
                    # Found a back-edge to a previously visited ancestor
                    low[node] = min(low[node], discovery_time[neighbor])
                    
        # 4. Start DFS from root node 0
        dfs(0, -1)
        return bridges
