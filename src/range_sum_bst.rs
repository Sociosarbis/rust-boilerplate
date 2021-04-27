use super::Solution;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
      let mut stack = vec![];
      let mut cur = root;
      let mut ret = 0;
      while let Some(node_ref) = cur {
        if node_ref.borrow().val < low {
          cur = node_ref.borrow().right.clone();
        } else if node_ref.borrow().val > high {
          cur = node_ref.borrow().left.clone();
        } else {
          ret += node_ref.borrow().val;
          if node_ref.borrow().right.is_some() {
            stack.push(node_ref.borrow().right.clone());
          }
          cur = node_ref.borrow().left.clone();
        }

        if cur.is_none() && !stack.is_empty() {
          cur = stack.pop().unwrap();
        }
      }
      ret
    }
}