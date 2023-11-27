#include <vector>
#include <queue>
#include <utility>

using namespace std;
class Graph {
public:
    Graph(int n, vector<vector<int>>& edges): adj_list(n, vector<pair<int, int>>()) {
        for (auto& e: edges) {
            adj_list[e[0]].push_back({e[1], e[2]});
        }
    }
    
    void addEdge(vector<int> edge) {
        adj_list[edge[0]].push_back({edge[1], edge[2]});
    }
    
    int shortestPath(int node1, int node2) {
        vector<int> dist(adj_list.size(), INT_MAX);
        dist[node1] = 0;
        priority_queue<pair<int, int>> pq;
        pq.push({0, node1});
        while(!pq.empty()) {
            pair<int, int> curr = pq.top();
            pq.pop();
            int curr_cost = -curr.first;
            int curr_node = curr.second;
            if (curr_node == node2) {
                return curr_cost;
            }
            if (curr_cost > dist[curr_node]) {
                continue;
            }
            for (auto neighbor: adj_list[curr_node]) {
                int new_node = neighbor.first;
                int new_cost = curr_cost + neighbor.second;
                if (new_cost < dist[new_node]) {
                    dist[new_node] = new_cost;
                    pq.push({-new_cost, new_node});
                }
            }
        }
        return -1;
    }
private:
    vector<vector<pair<int, int>>> adj_list;
};

/**
 * Your Graph object will be instantiated and called as such:
 * Graph* obj = new Graph(n, edges);
 * obj->addEdge(edge);
 * int param_2 = obj->shortestPath(node1,node2);
 */