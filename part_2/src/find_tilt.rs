use super::*;

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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
      let mut ret = 0;
      Solution::find_tilt_dfs(&root, &mut ret);
      ret
    }

    fn find_tilt_dfs(node: &Option<Rc<RefCell<TreeNode>>>, ret: &mut i32) -> i32 {
      if let Some(node_ref) = node {
        let n = node_ref.borrow();
        let left = Solution::find_tilt_dfs(&n.left, ret);
        let right = Solution::find_tilt_dfs(&n.right, ret);
        *ret += (left - right).abs();
        n.val + left + right
      } else {
        0
      }
    }
}