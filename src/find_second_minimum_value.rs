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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
      let mut ret = -1;
      let min = root.clone().unwrap().borrow().val;
      let mut cur = root;
      let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
      while let Some(node_ref) = cur {
        let mut next_cur: Option<Rc<RefCell<TreeNode>>> = None;
        if let Some(left_ref) = &node_ref.borrow().left {
          let left = left_ref.borrow();
          if left.val > min && (ret == -1 || left.val < ret) {
            ret = left.val;
          } else if left.val == min {
            next_cur = Some(left_ref.clone());
          }
        }

        if let Some(right_ref) = &node_ref.borrow().right {
          let right = right_ref.borrow();
          if right.val > min && (ret == -1 || right.val < ret) {
            ret = right.val;
          } else if right.val == min {
            if next_cur.is_none() {
              next_cur = Some(right_ref.clone());
            } else {
              stack.push(Some(right_ref.clone()));
            }
          }
        }
        if next_cur.is_none() && !stack.is_empty() {
          next_cur = stack.pop().unwrap();
        }
        cur = next_cur;
      }
      ret
    }
}