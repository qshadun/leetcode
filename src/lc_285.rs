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
struct Solution;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_successor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ans = None;
        let mut nodeOpt = root;
        let p_val = p.unwrap().borrow().val;
        while let(Some(node)) = &nodeOpt {
            let next = if node.borrow().val <= p_val {
                node.borrow().right.clone()
            } else {
                ans = nodeOpt.clone();
                node.borrow().left.clone()
            };
            nodeOpt = next;
        }
        ans
    }
}