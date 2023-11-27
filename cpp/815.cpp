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
        int n = routes.size();
        for (auto &route : routes)
        {
            sort(route.begin(), route.end());
        }
        vector<vector<int>> bus_to_bus(n, vector<int>{});
        create_graph(bus_to_bus, routes);

        unordered_set<int> visited;
        deque<int> dq;
        for (int i = 0; i != n; ++i)
        {
            if (stop_at(routes[i], source))
            {
                visited.insert(i);
                dq.push_back(i);
            }
        }
        int step = 1;
        while (!dq.empty())
        {
            int cur_size = dq.size();
            for (int i = 0; i != cur_size; ++i)
            {
                int bus = dq.front();
                dq.pop_front();
                if (stop_at(routes[bus], target))
                {
                    return step;
                }
                for (int next_bus : bus_to_bus[bus])
                {
                    if (visited.find(next_bus) == visited.end())
                    {
                        visited.insert(next_bus);
                        dq.push_back(next_bus);
                    }
                }
            }
            ++step;
        }
        return -1;
    }

    void create_graph(vector<vector<int>> &bus_to_bus, const vector<vector<int>> &routes)
    {
        for (int i = 0; i != routes.size() - 1; ++i)
        {
            for (int j = i + 1; j < routes.size(); ++j)
            {
                if (has_common_stop(routes[i], routes[j]))
                {
                    bus_to_bus[i].push_back(j);
                    bus_to_bus[j].push_back(i);
                }
            }
        }
    }
    bool has_common_stop(const vector<int> &route1, const vector<int> &route2)
    {
        int i = 0, j = 0;
        while (i < route1.size() && j < route2.size())
        {
            if (route1[i] == route2[j])
            {
                return true;
            }
            if (route1[i] < route2[j])
            {
                ++i;
            }
            else
            {
                ++j;
            }
        }
        return false;
    }

    bool stop_at(const vector<int> route, const int stop)
    {
        for (int s : route)
        {
            if (s == stop)
            {
                return true;
            }
        }
        return false;
    }
};