#include <vector>
#include <algorithm>
#include <unordered_set>
#include <deque>
using namespace std;

class Solution
{
public:
    int numBusesToDestination(vector<vector<int>> &routes, int source, int target)
    {
        
        if (source == target)
        {
            return 0;
        }
        short MAX_STEP = 501;
        short dist[1000001];
        std::fill_n(dist, 1000001, MAX_STEP);
        dist[source] = 0;
        unordered_set<int> visited;
        int count = 0;
        while (visited.size() < routes.size() && count < routes.size())
        {
            for (int bus = 0; bus < routes.size(); ++bus)
            {
                if (visited.find(bus) != visited.end())
                {
                    continue;
                }
                short min_dist = MAX_STEP;
                for (int stop : routes[bus])
                {
                    min_dist = min(min_dist, (short)(dist[stop] + (short)1));
                }
                if (min_dist < MAX_STEP)
                {
                    visited.insert(bus);
                    for (int stop : routes[bus])
                    {
                        dist[stop] = min(min_dist, dist[stop]);
                    }
                }
            }
            count += 1;
        }
        return dist[target] == MAX_STEP ? -1 : dist[target];
    }
};