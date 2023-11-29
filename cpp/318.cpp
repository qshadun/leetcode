#include <unordered_map>
using namespace std;

class Solution {
public:
    int maxProduct(vector<string>& words) {
        unordered_map<int, int> bitmask_max_len;
        for (string word: words) {
            int bitmask = 0;
            for (char c: word) {
                bitmask |= 1 << (c - 'a');
            }
            if (word.size() > bitmask_max_len[bitmask]) {
                bitmask_max_len[bitmask] = word.size();
            }
        }
        int ans = 0;
        for (const auto& elem1: bitmask_max_len) {
            for (const auto& elem2: bitmask_max_len) {
                if ((elem1.first & elem2.first) == 0) {
                    ans = max(ans, elem1.second * elem2.second);
                }
            }
        }
        return ans;
        
    }
};