

mod lc_815_1;
mod lc_2785;
mod lc_1930;
mod lc_251;
mod lc_346;
mod lc_257;
mod lc_259;
mod lc_1841;
mod lc_285;
mod lc_1424;
mod lc_1630;
mod lc_288;
mod lc_296;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
struct Solution;

use std::cmp;
impl Solution {
    fn calc(x: i32, y: i32, op: char) -> i32 {
        if (op == '+') {
            x + y
        } else if (op == '-') {
            x - y
        } else {
            x * y
        }
    }
    fn _diff_ways_to_compute(expression: &str) -> Vec<i32> {
        let mut isNumber = true;
        let mut ans: Vec<i32> = Vec::new();
        for (i, c) in expression.char_indices() {
            if !c.is_digit(10) {
                isNumber = false;
                let left = Solution::_diff_ways_to_compute(&expression[0..i]);
                let right = Solution::_diff_ways_to_compute(&expression[i + 1..]);
                for x in left.iter() {
                    for y in right.iter() {
                        ans.push(Solution::calc(*x, *y, c));
                    }
                }
            }
        }
        if isNumber {
            ans.push(expression.parse::<i32>().unwrap());
        }
        return ans;

    }
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        Solution::_diff_ways_to_compute(&expression[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let sol = Solution;
        let ans = Solution::diff_ways_to_compute(String::from("2-1-1"));
        println!("{:?}", ans);
    }
}
