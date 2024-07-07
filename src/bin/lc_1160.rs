struct Solution;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut chars_count = vec![0; 26];
        for c in chars.bytes() {
            chars_count[(c - b'a') as usize] += 1;
        }
        let mut ans = 0;
        for w in words {
            let mut w_count = vec![0; 26];
            for c in w.bytes() {
                w_count[(c - b'a') as usize] += 1;
            }
            if w_count.iter().zip(&chars_count).all(|(&x, &y)| x <= y) {
                ans += w.len();
            }
        }
        ans as i32
    }
}

fn main() {

}