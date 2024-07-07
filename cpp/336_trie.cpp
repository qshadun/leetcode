#include <vector>
#include <unordered_map>
#include <algorithm>
#include <string>
#include <iostream>
using namespace std;

struct TrieNode {
    TrieNode* next[26];
    vector<int> suffix_palindrome_indexes;
    int end_index;

    TrieNode():next{}, suffix_palindrome_indexes{}, end_index(-1) {
    }
   
};

class Solution {
public:
    vector<vector<int>> palindromePairs(vector<string>& words) {
        TrieNode trie;
        for (int i = 0; i < words.size(); ++i) {
            string w = words[i];
            reverse(w.begin(), w.end());
            TrieNode* cur_node = &trie;
            for (int j = 0; j < w.size(); ++j) {
                if (is_suffix_palindrome(w, j)) {
                    cur_node ->suffix_palindrome_indexes.push_back(i);
                }
                int next_idx = w[j] - 'a';
                if (cur_node -> next[next_idx] == nullptr) {
                    cur_node -> next[next_idx] = new TrieNode;
                }
                cur_node = cur_node->next[next_idx];
            }
            cur_node -> end_index = i;
        }
        vector<vector<int>> ans;
        for (int i = 0; i < words.size(); ++i) {
            string w = words[i];
            TrieNode* cur_node = &trie;
            for (int j = 0; j < w.size(); ++j) {
                if (cur_node->end_index != -1 && is_suffix_palindrome(w, j)) {
                    ans.push_back({i, cur_node->end_index});
                }
                cur_node = cur_node -> next[w[j] - 'a'];
                if (cur_node == nullptr) break;
            }
            if (cur_node != nullptr) {
                if (cur_node->end_index != -1 && cur_node->end_index != i) {
                    ans.push_back({i, cur_node->end_index});
                }
                for (int j: cur_node->suffix_palindrome_indexes) {
                    ans.push_back({i, j});
                }
            }
        }

        return ans;
    }

    bool is_suffix_palindrome(const string &w, int start) {
        int end = w.size() - 1;
        while (start < end) {
            if (w[start] != w[end]) {
                return false;
            }
            start++;
            end--;
        }
        return true;
    }
    
};

int main() {
    Solution sol;
    vector<string> words{"a",""};
    vector<vector<int>> res = sol.palindromePairs(words);
    for (vector<int> p: res) {
        cout << p[0] << ',' << p[1] << endl;
    }
}