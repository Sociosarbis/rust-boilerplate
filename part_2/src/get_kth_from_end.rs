use super::*;

// Definition for singly-linked list.
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
  pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut n = 0;
    let mut cur = head.to_owned();
    while let Some(node_ref) = cur {
      n += 1;
      cur = node_ref.next;
    }
    n -= k;
    cur = head;
    for _ in 1..n + 1 {
      cur = cur.unwrap().next;
    }
    cur
  }
}