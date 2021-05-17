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
use super::Solution;

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
      let mut nodes = vec![root];
      let mut first = -1;
      while !nodes.is_empty() {
        let mut new_nodes = vec![];
        for i in 0..nodes.len() {
          if let Some(node_ref) = &nodes[i] {
            let node = node_ref.borrow();
            if first == -1 {
              if node.val == x || node.val == y {
                first = i as i32;
              }
            } else {
              if node.val == x || node.val == y {
                if i as i32 == first + 1 && first & 1 == 0 {
                  return false;
                } else {
                  return true;
                }
              }
            }
            new_nodes.push(node.left.clone());
            new_nodes.push(node.right.clone());
          }
        }
        if first != -1 {
          return false;
        }
        nodes = new_nodes;
      }
      false
    }
}