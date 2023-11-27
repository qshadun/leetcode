struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let n = s.len();
        let mut lefts = vec![n; 26];
        let mut rights = vec![0; 26];
        for (i, c) in s.char_indices() {
            let idx = c as usize - 'a' as usize;
            if lefts[idx] == n {
                lefts[idx] = i;
            }
            rights[idx] = i;
        }
        let mut ans = 0;
        for i in 0..26 {
            if lefts[i] + 1 < rights[i] {
                let unique_chars: HashSet<char> = s[lefts[i]+1..rights[i]].chars().collect();
                ans += unique_chars.len();
            }
        }
        
        ans as i32
    }
}