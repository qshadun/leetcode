#include <vector>
#include <map>
using namespace std;
class Solution {
public:
    vector<int> restoreArray(vector<vector<int>>& adjacentPairs) {
        unordered_map<int, vector<int>> graph;
        for (auto& edge: adjacentPairs) {
            if (graph.find(edge[0]) == graph.end()) {
                graph[edge[0]] = {edge[1]};
            } else {
                graph[edge[0]].push_back(edge[1]);
            }

            if (graph.find(edge[1]) == graph.end()) {
                graph[edge[1]] = {edge[0]};
            } else {
                graph[edge[1]].push_back(edge[0]);
            }
        }

        vector<int> ans;
        int prev = 0, cur = 0;
        for (auto& entry: graph) {
            if (entry.second.size() == 1) {
                prev = entry.first;
                cur = entry.second[0];
                ans.push_back(prev);
                ans.push_back(cur);
            }
        }

        while (ans.size() < adjacentPairs.size() + 1) {
            for (auto& num: graph[cur]) {
                if (num != prev) {
                    prev = cur;
                    cur = num;
                    ans.push_back(cur);
                    break;
                }
            }
        }
        return ans; 
    }
};