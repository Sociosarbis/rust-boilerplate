use super::*;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}
//
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
  pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    let mut cur = head.clone();
    let mut count = 0;
    while let Some(node_ptr) = cur {
      count += 1;
      cur = node_ptr.next;
    }
    let num_per_part = count / k;
    let mut rest = count - num_per_part * k;
    let mut ret = vec![];
    cur = head.clone();
    for _ in 0..k {
      let mut node = cur.clone();
      let mut node_ref = node.as_mut();
      let n = num_per_part + if rest != 0 { 1 } else { 0 };
      for i in 0..n {
        cur = cur.unwrap().next;
        if i + 1 == n {
          node_ref.unwrap().next = None;
          break
        } else {
          node_ref = node_ref.unwrap().next.as_mut();
        }
      }
      ret.push(node);
      if rest != 0 {
        rest -= 1;
      }
    }
    ret
  }
}