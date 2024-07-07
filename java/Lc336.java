import java.util.*;

public class Lc336 {

}

class TrieNode {
    TrieNode[] next = new TrieNode[26];
    List<Integer> suffixPalindromeIndexes = new ArrayList<>();
    int end_index = -1;
}
class Solution {
    public List<List<Integer>> palindromePairs(String[] words) {
        TrieNode trie = new TrieNode();
        for (int i = 0; i < words.length; i++) {
            String w = new StringBuilder(words[i]).reverse().toString();
            TrieNode curNode = trie;
            for (int j = 0; j < w.length(); j++) {
                char c = w.charAt(j);
                if (isPalindromeBetween(w, j, w.length() - 1)) {
                    curNode.suffixPalindromeIndexes.add(i);
                }
                if (curNode.next[c - 'a'] == null){
                    curNode.next[c - 'a'] = new TrieNode();
                }
                curNode = curNode.next[c - 'a'];
            }
            curNode.end_index = i;
        }

        List<List<Integer>> ans = new ArrayList<>();
        for (int i = 0; i < words.length; i++) {
            String w = words[i];
            TrieNode curNode = trie;
            for (int j = 0; j < w.length(); j++) {
                if (curNode.end_index != -1 && isPalindromeBetween(w, j, w.length() - 1)) {
                    ans.add(Arrays.asList(i, curNode.end_index));
                }
                curNode = curNode.next[w.charAt(j) - 'a'];
                if (curNode == null) break;
            }
            if (curNode != null) {
                if (curNode.end_index != -1 && curNode.end_index != i) {
                    ans.add(Arrays.asList(i, curNode.end_index));
                }
                for (int j: curNode.suffixPalindromeIndexes) {
                    ans.add(Arrays.asList(i, j));
                }
            }
        }
        return ans;
    }

    private boolean isPalindromeBetween(String w, int start, int end) {
        while (start < end) {
            if (w.charAt(start) != w.charAt(end)) {
                return false;
            }
            start++;
            end--;
        }
        return true;
    }
}