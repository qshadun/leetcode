#include <string>
#include <unordered_set>
using namespace std;
class Solution {
public:
    int countPalindromicSubsequence(string s) {
        int lefts[26];
        fill_n(lefts, 26, -1);
        int rights[26]{};
        int n = s.length();
        for (int i = 0; i < n; i++){
            int idx = s[i] - 'a';
            if (lefts[idx] == -1) {
                lefts[idx] = i;
            }
            rights[idx] = i;
        }

        int ans = 0;
        for (int i = 0; i < 26; i++) {
            if (lefts[i] + 1 < rights[i]) {
                ans += unordered_set<char>(s.begin() + lefts[i] + 1, s.begin() + rights[i]).size();
            }
        }
        return ans;
    }
};

