class Solution:
    def cherryPickup(self, grid: List[List[int]]) -> int:
        def best_path(grid):
            n = len(grid)
            NINF = float('-inf')
            dp = [[NINF] * (n+1) for _ in range(n + 1)]
            dp[n-1][n-1] = grid[n-1][n-1]
            for i in range(n-1, -1, -1):
                for j in range(n-1, -1, -1):
                    if grid[i][j] >= 0 and not (i == n-1 and j == n-1):
                        dp[i][j] = max(dp[i+1][j], dp[i][j+1])
                        dp[i][j] += grid[i][j]
            if dp[0][0] < 0:
                return None
            ans = [(0, 0)]
            i, j = 0, 0
            while not (i == n-1 and j == n-1):
                if j == n - 1 or i < n - 1 and dp[i+1][j] >= dp[i][j+1]:
                    i += 1
                else:
                    j += 1
                ans.append((i, j))
            return ans
        
        ans = 0
        path = best_path(grid)
        if path is None:
            return 0
        
        for i, j in path:
            ans += grid[i][j]
            grid[i][j] = 0
        
        for i, j in best_path(grid):
            ans += grid[i][j]
        return ans