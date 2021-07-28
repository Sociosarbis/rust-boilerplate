use super::Solution;

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
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> Vec<i32> {
        let target_val = target.unwrap().borrow().val;
        let mut root_to_target: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        Solution::find_target(root, target_val, &mut root_to_target);
        let mut visited = vec![false;501];
        let mut ret = vec![];
        while !root_to_target.is_empty() && k >= 0 {
          if let Some(cur) = root_to_target.pop() {
            Solution::distance_k_dfs(cur.clone(), k, &mut visited, &mut ret);
          }
          k -= 1;
        }
        ret
    }

    fn find_target(node: Option<Rc<RefCell<TreeNode>>>, target_val: i32, list: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) -> bool {
      if let Some(node_ref) = node {
        list.push(Some(node_ref.clone()));
        let val = node_ref.borrow().val;
        if val == target_val {
          return true;
        } else {
          if Solution::find_target(node_ref.borrow().left.clone() , target_val, list) {
            return true;
          }
  
          if Solution::find_target(node_ref.borrow().right.clone() , target_val, list) {
            return true;
          }
          list.pop();
        }
      }
      return false;
    }

    fn distance_k_dfs(node: Option<Rc<RefCell<TreeNode>>>, k: i32, visited: &mut [bool], ret: &mut Vec<i32>) {
      if let Some(node_ref) = node {
        let val = node_ref.borrow().val as usize;
        if !visited[val] {
          visited[val] = true;
          if k == 0 {
            ret.push(node_ref.borrow().val);
          } else {
            Solution::distance_k_dfs(node_ref.borrow().left.clone(), k - 1, visited, ret);
            Solution::distance_k_dfs(node_ref.borrow().right.clone(), k - 1, visited, ret);
          }
        }
      }
    }
}