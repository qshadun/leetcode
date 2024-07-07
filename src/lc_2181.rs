struct Solution;


struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        let mut ans = Box::new(ListNode::new(0));
        let mut cur_node = &mut ans;
        let mut head = head.as_ref();

        while let Some(node) = head {
            head = node.next.as_ref();
            if node.val == 0 {
                if node.next.is_some() {
                    cur_node.next = Some(Box::new(ListNode::new(0)));
                    cur_node = cur_node.next.as_mut().unwrap();
                } else {
                    break;
                }
                
            } else {
                cur_node.val += node.val;
            }
        }
        
        ans.next
    }
}

