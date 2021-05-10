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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
      let mut seq1 = vec![];
      let mut seq2 = vec![];
      Solution::leaf_similar_dfs(&root1, &mut seq1);
      Solution::leaf_similar_dfs(&root2, &mut seq2);
      if seq1.len() != seq2.len() {
        return false;
      }
      for i in 0..seq1.len() {
        if seq1[i] != seq2[i] {
          return false
        }
      }
      true
    }

    fn leaf_similar_dfs(node: &Option<Rc<RefCell<TreeNode>>>, seq: &mut Vec<i32> ) {
      if let Some(node_ref) = node {
        let parent = node_ref.borrow();
        if parent.left.is_none() && parent.right.is_none() {
          seq.push(parent.val);
          return
        }
        Solution::leaf_similar_dfs(&parent.left, seq);
        Solution::leaf_similar_dfs(&parent.right, seq);
      }
    }
}