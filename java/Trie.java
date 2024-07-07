class Trie {
    private Trie[] children = new Trie[26];
    private boolean isEnd = false;
    public Trie() {
        
    }
    
    public void insert(String word) {
        if (word.isEmpty()) return;
        Trie curNode = this;
        for (int i = 0; i < word.length(); i++) {
            int idx = word.charAt(i) - 'a';
            if (curNode.children[idx] == null) {
                curNode.children[idx] = new Trie();
            }
            curNode = curNode.children[idx];
        }
        curNode.isEnd = true;
    }
    
    public boolean search(String word) {
        Trie curNode = this;
        for (int i = 0; i < word.length(); i++) {
            int idx = word.charAt(i) - 'a';
            if (curNode.children[idx] == null) {
                return false;
            }
            curNode = curNode.children[idx];
        }
        return curNode.isEnd;
    }
    
    public boolean startsWith(String prefix) {
        Trie curNode = this;
        for (int i = 0; i < prefix.length(); i++) {
            int idx = prefix.charAt(i) - 'a';
            if (curNode.children[idx] == null) {
                return false;
            }
            curNode = curNode.children[idx];
        }
        return true;
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * Trie obj = new Trie();
 * obj.insert(word);
 * boolean param_2 = obj.search(word);
 * boolean param_3 = obj.startsWith(prefix);
 */