#include <vector>
#include <queue>
#include <tuple>
#include <iostream>
using namespace std;

class Solution {
public:
    int nthSuperUglyNumber(int n, vector<int>& primes) {
        
        typedef tuple<long long, int, int> elem;
        auto comp = [](elem t1, elem t2) {
            return get<0>(t1) > get<0>(t2);
        };
        priority_queue<elem, vector<elem>, decltype(comp)> pq(comp);
        for (int prime: primes) {
            pq.push({prime, prime, 0});
        }
        vector<int> nums(n, 0);
        nums[0] = 1;
        int i = 1;
        while (i < n) {
            auto [num, prime, idx] = pq.top();
            pq.pop();
            if (num != nums[i - 1]) {
                nums[i] = num;
                ++i;
            }
            pq.push({(long long)prime * nums[idx + 1], prime, idx + 1});
        }
        return nums[n - 1];
    }
};

int main() {
    Solution sol;
    vector<int> primes{2, 7, 13, 19};
    cout << sol.nthSuperUglyNumber(12, primes) << endl;;
}