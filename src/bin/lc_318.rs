struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut bitmask_max_len = HashMap::new();
        for word in words {
            let mut bitmask = 0;
            for c in word.chars() {
                bitmask |= 1 << (c as u8  - 'a' as u8);
            }
            bitmask_max_len.entry(bitmask).and_modify(|old_value| {
                if word.len() > *old_value {
                    *old_value = word.len();
                }
            }).or_insert(word.len());
        }
        let mut ans: usize = 0;
        for (bitmask1, max_len1) in &bitmask_max_len {
            for (bitmask2, max_len2) in &bitmask_max_len {
                if bitmask1 & bitmask2 == 0 {
                    ans = std::cmp::max(ans, max_len1 * max_len2);
                }
            }
        }
        ans as i32
    }
}

fn main() {

}