struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bs = s.as_bytes();
        if bs[0] == b'0' {
            return 0;
        }
        let mut one_back = 1;
        let mut two_back = 1;
        for i in (1..bs.len()) {
            let mut cur = 0;
            if bs[i] != b'0' {
                cur = one_back;
            }
            if bs[i - 1] == b'1' || (bs[i - 1] == b'2' && bs[i] <= b'6') {
                cur += two_back;
            }
            two_back = one_back;
            one_back = cur;
        }
        one_back
    }
}

fn main() {

}