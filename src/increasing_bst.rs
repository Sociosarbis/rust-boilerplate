use super::Solution;
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
      let mut stack = vec![];
      let mut cur = root;
      let ret = Some(Rc::new(RefCell::new(TreeNode::new(0))));
      let mut tail = ret.clone();
      while let Some(node_ref) = cur.clone() {
        let left_node = {
          node_ref.borrow().left.clone()
        };
        if let Some(left_node_ref) = left_node {
          node_ref.borrow_mut().left = None;
          stack.push(cur);
          cur = Some(left_node_ref);
        } else {
          tail.clone().unwrap().borrow_mut().right = Some(node_ref);
          tail = tail.unwrap().borrow().right.clone();
          cur = tail.clone().unwrap().borrow().right.clone();
        }
        while cur.is_none() && !stack.is_empty() {
          tail.clone().unwrap().borrow_mut().right = stack.pop().unwrap();
          tail = tail.unwrap().borrow().right.clone();
          cur = tail.clone().unwrap().borrow().right.clone();
        }
      }
      ret.unwrap().borrow().right.clone()
    }
}