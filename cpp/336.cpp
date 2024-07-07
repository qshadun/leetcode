#include <vector>
#include <unordered_map>
#include <algorithm>
#include <string>
#include <iostream>
using namespace std;

class Solution {
public:
    vector<vector<int>> palindromePairs(vector<string>& words) {
        unordered_map<string, int> word_dict;
        for (int i = 0; i < words.size(); ++i) {
            word_dict[words[i]] = i;
        }
        vector<vector<int>> ans;
        for (int i = 0; i < words.size(); ++i) {
            string rev = words[i];
            reverse(rev.begin(), rev.end());
            if (word_dict.find(rev) != word_dict.end() && word_dict[rev] != i) {
                ans.push_back({i, word_dict[rev]});
            }

            vector<string> prefixes;
            get_valid_prefixes(words[i], prefixes);
            for (string &prefix: prefixes) {
                reverse(prefix.begin(), prefix.end());
                if (word_dict.find(prefix) != word_dict.end()) {
                    ans.push_back({i, word_dict[prefix]});
                }
            }

            vector<string> suffixes;
            get_valid_suffixes(words[i], suffixes);
            for (string &suffix: suffixes) {
                reverse(suffix.begin(), suffix.end());
                if (word_dict.find(suffix) != word_dict.end()) {
                    ans.push_back({word_dict[suffix], i});
                }
            }
        }
        return ans;
    }

    bool is_palindrome_between(string &w, int start, int end) {
        while (start < end) {
            if (w[start] != w[end]) {
                return false;
            }
            start++;
            end--;
        }
        return true;
    }
    void get_valid_suffixes(string &word, vector<string> &suffixes) {
        for (int i = 0; i < word.size(); i++) {
            if (is_palindrome_between(word, 0, i)) {
                suffixes.push_back(word.substr(i + 1, word.size() - i));
            }
        }
    }

    void get_valid_prefixes(string &word, vector<string> &prefixes) {
        for (int i = 0; i < word.length(); i++) {
            if (is_palindrome_between(word, i, word.length() - 1)) {
                prefixes.push_back(word.substr(0, i));
            }
        }
    }
};

int main() {
    string s1 = "abc";
    string s2 = s1;
    reverse(s2.begin(), s2.end());
    cout << "s1:" << s1 <<endl;
    cout << "s2:" << s2 << endl;
}