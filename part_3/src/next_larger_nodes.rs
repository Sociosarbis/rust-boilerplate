use super::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


impl Solution {
  pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut stack: Vec<(usize, i32)> = vec![];
    let mut ret = vec![];
    let mut i = 0;
    while let Some(node) = head {
      if !stack.is_empty() {
        for i in (0..stack.len()).rev() {
          if stack[i].1 < node.val {
            ret[stack[i].0] = node.val;
            stack.pop();
          } else {
            break;
          }
        }
      }
      stack.push((i, node.val));
      ret.push(0);
      i += 1;
      head = node.next;
    }
    ret
  }
}