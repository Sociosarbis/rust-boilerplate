use super::*;

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
use std::collections::HashMap;


fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, nums: &mut HashMap<i32, i32>) {
  if let Some(n) = node {
    let num = n.borrow();
    dfs(&num.left, nums);
    if let Some(c) = nums.get_mut(&num.val) {
      *c += 1;
    } else {
      nums.insert(num.val, 1);
    }
    dfs(&num.right, nums);
  }
}

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
      let mut nums = HashMap::new();
      dfs(&root, &mut nums);
      for (&key, &val) in &nums {
        let target = k - key;
        if target == key {
          if val > 1 {
            return true;
          }
        } else {
          if let Some(_) = nums.get(&target) {
            return true;
          }
        }
      }
      false
    }
}