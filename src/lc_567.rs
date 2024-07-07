struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (m, n) = (s1.len(), s2.len());
        if m > n {
            return false;
        }
        let mut s1_counter: Vec<i32> = vec![0; 26];
        let mut s2_counter: Vec<i32> = vec![0; 26];
        for c in s1.chars() {
            s1_counter[c as usize - 'a' as usize] += 1;
        }
        for c in s2.chars().take(m) {
            s2_counter[c as usize - 'a' as usize] += 1;
        }
        let mut made = 0;
        for i in 0..26 {
            if s1_counter[i] == s2_counter[i] {
                made += 1;
            }
        }
        if made == 26 {
            return true;
        }
        let s2_chars: Vec<char> = s2.chars().collect();
        for i in m..n {
            let idx_add = s2_chars[i] as usize - 'a' as usize;
            let idx_remove = s2_chars[i-m] as usize - 'a' as usize;
            s2_counter[idx_add] += 1;
            if s2_counter[idx_add] == s1_counter[idx_add] {
                made += 1;
            } else if s2_counter[idx_add] == s1_counter[idx_add] + 1 {
                made -= 1;
            }
            s2_counter[idx_remove] -= 1;
            if s2_counter[idx_remove] == s1_counter[idx_remove] {
                made += 1;
            } else if s2_counter[idx_remove] == s1_counter[idx_remove] - 1 {
                made -= 1;
            }
            if made == 26 {
                return true;
            }
        }
        false
    }
}