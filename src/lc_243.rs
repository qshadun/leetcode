struct Solution;
use std::cmp;
impl Solution {
    pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        let n = words_dict.len();
        let mut i1 = n;
        let mut i2 = n;
        let mut ans = n;
        for i in 0..n {
            if words_dict[i] == word1 {
                i1 = i;
                if i2 != n {
                    ans = cmp::min(ans, i1 - i2);
                }
            } else if words_dict[i] == word2 {
                i2 = i;
                if i1 != n {
                    ans = cmp::min(ans, i2 - i1);
                }
            }
        }
        ans as i32
    }
}