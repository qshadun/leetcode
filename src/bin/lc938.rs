#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut ans = 0;
        if let Some(node) = root {
            let node = node.borrow();
            let val = node.val;
            if val <= high && val >= low {
                ans += val;
            }
            if val >= low {
                ans += Self::range_sum_bst(node.left.clone(), low, high);
            }
            if val <= high {
                ans += Self::range_sum_bst(node.right.clone(), low, high);
            }
        }
        ans
    }
}

fn main() {

}