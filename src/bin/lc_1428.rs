
struct BinaryMatrix;
impl BinaryMatrix {
   fn get(&self, row: i32, col: i32) -> i32 {
    2
   }
   fn dimensions(&self) -> Vec<i32>{
    vec![1, 2]
   }
}

struct Solution;

impl Solution {
    pub fn left_most_column_with_one(binaryMatrix: &BinaryMatrix) -> i32 {
        let dim = binaryMatrix.dimensions();
        let m = dim[0];
        let n = dim[1];
        let mut ans = n;
        for i in 0..m {
            if ans < n && binaryMatrix.get(i, ans) == 0 {
                continue;
            }
            let mut l = 0;
            let mut r = ans - 1;
            while l <= r {
                let mid = l + (r - l) / 2;
                let val = binaryMatrix.get(i, mid);
                if val == 0 {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
            if l < ans {
                ans = l;
            }
        }
        if ans < n {ans} else {-1}
         
    }
}

fn main() {

}