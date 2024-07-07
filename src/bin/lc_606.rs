struct Solution;
 pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
 }

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let node_rc = root.unwrap();
        let mut node_ref = node_rc.borrow_mut();
        
        node_ref.val.to_string()
            + &match (node_ref.left.take(), node_ref.right.take()) {
                (None, None) => String::new(),
                (left, None) => format!("({})", Self::tree2str(left)),
                (None, right) => format!("()({})", Self::tree2str(right)),
                (left, right) => format!("({})({})", Self::tree2str(left), Self::tree2str(right)),
            }
    }
}

fn main() {

}