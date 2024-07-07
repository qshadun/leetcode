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

type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn leaf_similar(root1: Tree, root2: Tree) -> bool {
        leaves(root1) == leaves(root2)
    }
}

pub fn leaves(root: Tree) -> Vec<i32> {
    let mut stack: Vec<Tree> = vec![];
    stack.push(root);
    let mut ans = vec![];
    while let Some(node) = stack.pop() {
        let node = node.as_ref().unwrap().borrow();
        if node.left.is_none() && node.right.is_none() {
            ans.push(node.val);
        }
        if node.left.is_some() {
            stack.push(node.left.clone());
        }
        if node.right.is_some() {
            stack.push(node.right.clone());
        }
        
    }
    ans
}

fn main() {

}