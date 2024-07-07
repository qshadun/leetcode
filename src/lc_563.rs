// Definition for a binary tree node.
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

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match node {
                Some(node) => {
                    let left = dfs(&node.borrow().left);
                    let right = dfs(&node.borrow().right);
                    (left.0 + right.0 + node.borrow().val, (left.0 - right.0).abs() + left.1 + right.1)
                }
                None => (0, 0)
            }
        }
        dfs(&root).1
    }
}