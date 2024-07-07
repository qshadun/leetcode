#include <string>
using namespace std;

class Trie {
public:
    Trie():children{}, is_end(false) {
        
    }
    
    void insert(string word) {
        if (word.empty()) return;
        Trie* cur_node = this;
        for (char c: word) {
            int idx = c - 'a';
            if (cur_node->children[idx] == nullptr) {
                cur_node->children[idx] = new Trie();
            }
            cur_node = cur_node->children[idx];
        }
        cur_node->is_end = true;
    }
    
    bool search(string word) {
        Trie* cur_node = this;
        for (char c: word) {
            int idx = c - 'a';
            if (cur_node->children[idx] == nullptr) {
                return false;
            }
            cur_node = cur_node->children[idx];
        }
        return cur_node->is_end;
    }
    
    bool startsWith(string prefix) {
        Trie* cur_node = this;
        for (char c: prefix) {
            int idx = c - 'a';
            if (cur_node->children[idx] == nullptr) {
                return false;
            }
            cur_node = cur_node->children[idx];
        }
        return true;
    }
private:
    Trie* children[26];
    bool is_end;
};

/**
 * Your Trie object will be instantiated and called as such:
 * Trie* obj = new Trie();
 * obj->insert(word);
 * bool param_2 = obj->search(word);
 * bool param_3 = obj->startsWith(prefix);
 */