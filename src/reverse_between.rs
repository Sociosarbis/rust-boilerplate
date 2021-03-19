use super::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  #[allow(dead_code)]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
      let mut prev = None;
      let mut origin = Some(Box::new(ListNode { val: 0, next: head }));
      let mut cur = origin.as_mut();
      let mut i = 1;
      while let Some(mut node) = cur {
        if i == left {
          let mut temp = node.next.take();
          while i >= left && i <= right {
            if let Some(mut take_node) = temp.take() {
              let next_node = take_node.next.take();
              take_node.next = prev.take();
              prev = Some(take_node);
              i += 1;
              temp = next_node;
            }
          }
          let mut next = prev;
          let mut next_mut = next.as_mut();
          while let Some(a) = next_mut {
            if a.next.is_none() {
              a.next = temp.take();
            }
            next_mut = a.next.as_mut();
          }
          node.next = next;
          break;
        }
        i += 1;
        cur = node.next.as_mut();
      }
      origin.unwrap().next
    }
}