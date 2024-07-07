#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    bool checkInclusion(string s1, string s2) {
        int m = s1.size(), n= s2.size();
        if (m > n) return false;
        int s1_counter[26] = {0};
        int s2_counter[26] = {0};
        for (int i = 0; i < m; i++) {
            s1_counter[s1[i] - 'a']++;
            s2_counter[s2[i] - 'a']++;
        }
        int made = 0;
        for (int i = 0; i < 26; i++) {
            if (s1_counter[i] == s2_counter[i]) {
                made++;
            }
        }
        if (made == 26) return true;
        for (int i = m; i < n; i++) {
            int idx_add = s2[i] - 'a';
            int idx_remove = s2[i-m] - 'a';
            s2_counter[idx_add]++;
            if (s2_counter[idx_add] == s1_counter[idx_add]) {
                made++;
            }else if (s2_counter[idx_add] == s1_counter[idx_add] + 1) {
                made--;
            }
            s2_counter[idx_remove]--;
            if (s2_counter[idx_remove] == s1_counter[idx_remove]) {
                made++;
            }else if (s2_counter[idx_remove] == s1_counter[idx_remove] - 1) {
                made--;
            }
            if (made == 26) return true; 
        }
        return false;
    }
};