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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
      let mut cur = root;
      let mut prev = -1;
      let mut ret = -1;
      let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
      while !cur.is_none() || !stack.is_empty() {
        if let Some(node) = cur.clone() {
          let _node = node.borrow();
          if _node.left.is_none() {
            if prev != -1 {
              let diff = _node.val - prev;
              if ret == -1 {
                ret = diff;
              } else {
                if diff < ret {
                  ret = diff;
                }
              }
            }
            prev = _node.val;
            cur = _node.right.clone();
          } else {
            stack.push(cur);
            cur = _node.left.clone();
          }
        }

        if cur.is_none() {
          while let Some(top) = stack.pop() {
            if let Some(_top_node) = top {
              let top_node = _top_node.borrow();
              let diff = top_node.val - prev;
              if ret == -1 {
                ret = diff;
              } else {
                if diff < ret {
                  ret = diff;
                }
              }
              prev = top_node.val;

              if !top_node.right.is_none() {
                cur = top_node.right.clone();
                break
              }
            }
          }
        }
        if ret == 0 {
          break
        }
      }
      ret
    }
}