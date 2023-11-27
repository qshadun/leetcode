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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result = vec![];
        Solution::dfs(&root, String::from(""), &mut result);
        result
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut path: String, result: &mut Vec<String>) {
        match node {
            Some(n) => {
                let nn = (**n).borrow();
                path = format!("{}{}", path, nn.val.to_string());
                if nn.left == None && nn.right == None {
                    result.push(path);
                } else {
                    Solution::dfs(&nn.left, format!("{}{}", path, "->"), result);
                    Solution::dfs(&nn.right, format!("{}{}", path, "->"), result);
                }
            }
            None => (),
        }
    }
}