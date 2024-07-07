#include <vector>
#include <unordered_set>
#include <utility>
#include <queue>
using namespace std;

#pragma GCC optimize("O3", "unroll-loops")
class Solution {
public:
    int networkDelayTime(vector<vector<int>>& times, int n, int k) {
        vector<vector<pair<int, int>>> adj_list{(size_t)n + 1, vector<pair<int, int>>{}};
        
        for (auto time: times ) {
            adj_list[time[0]].push_back({time[1], time[2]});
        }
        int ans = 0;
        
        int dist[n + 1];
        fill(dist + 1, dist + n + 1, INT_MAX);
        dist[k] = 0;
        
        priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> q;
        q.push({0, k});
        int connected = 0;
        while (!q.empty()) {
            auto [w, u] = q.top();
            q.pop();
            if (w > dist[u]) continue;
            ans = w;
            ++connected;
            for (auto [v, edge]: adj_list[u]) {
                int new_dist = w + edge;
                if (new_dist < dist[v]) {
                    dist[v] = new_dist;
                    q.push({new_dist, v});
                }
                
            }
        }
        return connected == n ? ans : -1;
    }
};