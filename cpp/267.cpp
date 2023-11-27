#include <vector>
#include <string>
#include <algorithm>
#include <iostream>
using namespace std;


class Solution {
public:
    vector<string> generatePalindromes(string s) {
        int counter[26]{};
        char odd_char = 0;
        for (char c: s) {
            counter[c - 'a'] += 1;
        }
        for (int i = 0; i < 26; i++) {
            if (counter[i] % 2 != 0) {
                if (odd_char != 0) {
                    return {};
                } else {
                    odd_char = 'a' + i;
                    counter[i] -= 1;
                }
            }
        }

        
        vector<string> result{};
        int half = s.size() / 2;
        string sofar;
        backtrack(result, odd_char, half, counter, sofar);
        return result;
    }

    void backtrack(vector<string> &result, char odd_char, int half, int* counter, string& sofar) {
        if (sofar.size() == half) {
                string head(sofar);
                string tail(sofar);
                reverse(tail.begin(), tail.end());
                if (odd_char != 0) {
                    head.push_back(odd_char);
                }
                result.push_back(head + tail);
            } else {
                for (int i = 0; i < 26; i++) {
                    if (counter[i] > 0) {
                        counter[i] -= 2;
                        sofar.push_back('a' + i);
                        backtrack(result, odd_char, half, counter, sofar);
                        sofar.pop_back();
                        counter[i] += 2;
                    }
                }
            }
    }
};

int main() {
    Solution sol;
    vector<string> ans = sol.generatePalindromes("aabb");
    for (string s: ans) {
        cout << s << endl;
    }
}