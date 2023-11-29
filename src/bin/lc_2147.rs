struct Solution;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut seat_positions = vec![];
        for (i, c) in corridor.char_indices() {
            if c == 'S' {
                seat_positions.push(i);
            }
        }
        if seat_positions.len() == 0 || seat_positions.len() % 2 != 0 {
            return 0;
        }
        let mut ans: usize = 1;
        let MOD = usize::pow(10, 9) + 7;
        for i in (2..seat_positions.len()).step_by(2) {
            ans = (ans * (seat_positions[i] - seat_positions[i - 1])) % MOD;
        }
        ans as i32
    }
}

fn main() {

}