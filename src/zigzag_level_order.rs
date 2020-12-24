use super::Solution;

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
      right: None,
    }
  }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
  pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut bfs: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
    let mut ret: Vec<Vec<i32>> = vec![];
    if root.is_none() {
      return ret;
    }
    let mut is_rev = false;
    bfs.push(root);
    while bfs.len() != 0 {
      let n = bfs.len();
      let mut tmp: Vec<i32> = vec![];
      let mut next_bfs: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
      for _ in 0..n {
        if let Some(opt) = bfs.pop() {
          if let Some(item) = opt {
            let node = item.borrow();
            if !is_rev {
              if !node.left.is_none() {
                next_bfs.push(node.left.clone());
              }
              if !node.right.is_none() {
                next_bfs.push(node.right.clone());
              }
            } else {
              if !node.right.is_none() {
                next_bfs.push(node.right.clone());
              }
              if !node.left.is_none() {
                next_bfs.push(node.left.clone());
              }
            }
            tmp.push(node.val);
          }
        }
      }
      bfs.append(&mut next_bfs);
      is_rev = !is_rev;
      ret.push(tmp);
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn make_node(val: Option<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if val.is_none() { None } else { Some(Rc::new(RefCell::new(TreeNode::new(val.unwrap())))) }
  }

  fn dfs(node: Option<Rc<RefCell<TreeNode>>>, i: usize, arr: &Vec<Option<i32>>) {
    let l = (i << 1) + 1;
    let r = l + 1;
    let inner = node.unwrap();
    if l < arr.len() {
      let left = make_node(arr[l]);
      if left.is_some() {
        inner.borrow_mut().left = left.clone();
        dfs(left, l, arr);
      }
    }
    if r < arr.len() {
      let right = make_node(arr[r]);
      if right.is_some() {
        inner.borrow_mut().right = right.clone();
        dfs(right, r, arr);
      }
    }
  }

  fn build_tree(arr: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let tree = make_node(arr[0]);
    dfs(tree.clone(), 0, &arr);
    tree
  }

  #[test]
  fn zigzag_level_order_simple() {
    let arr: Vec<Option<i32>> = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];

    assert_eq!(
      Solution::zigzag_level_order(build_tree(&arr)),
      vec![vec![3], vec![20, 9], vec![15, 7]]
    );
  }
}
