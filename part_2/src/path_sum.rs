use super::*;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
      Solution::path_sum_dfs(&root, 0, target_sum) 
      + if let Some(node_cell) = root.as_ref() { Solution::path_sum(node_cell.borrow().left.clone(), target_sum) } else { 0 }
      + if let Some(node_cell) = root.as_ref() { Solution::path_sum(node_cell.borrow().right.clone(), target_sum) } else { 0 }
    }

    fn path_sum_dfs(root: &Option<Rc<RefCell<TreeNode>>>, temp: i64, target_sum: i32) -> i32 {
      if let Some(node_cell) = root {
        let node_ref = node_cell.borrow();
        let new_temp = temp + node_ref.val as i64;
        return if new_temp == target_sum as i64 { 1 } else { 0 } 
          + Solution::path_sum_dfs(&node_ref.left, new_temp, target_sum)
          + Solution::path_sum_dfs(&node_ref.right, new_temp, target_sum)
      }
      0
    }
}