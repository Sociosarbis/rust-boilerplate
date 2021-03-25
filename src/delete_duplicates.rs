use super::Solution;

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
  pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut ret = Some(Box::new(ListNode::new(0)));
    let mut ret_mut = ret.as_mut();
    let mut cur = head.as_mut();
    while let Some(node) = cur {
      let val = node.val;
      let mut next = node.next.as_mut();
      let mut can_add = true;
      while let Some(next_node) = next {
        if next_node.val != val {
          next = Some(next_node);
          break;
        } else if can_add {
          can_add = false;
        }
        next = next_node.next.as_mut();
      }
      if can_add {
        if let Some(ret_node) = ret_mut {
          ret_node.next = Some(Box::new(ListNode::new(val)));
          ret_mut = ret_node.next.as_mut();
        }
      }
      cur = next;
    }
    ret.unwrap().next
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    head: Vec<i32>,
    ret: Vec<i32>
  }

  #[test]
  fn delete_duplicates_simple() {
    let suites = vec![
      Suite {
        head: vec![1,2,3,3,4,4,5],
        ret: vec![1,2,5]
      },
      Suite {
        head: vec![1,1,1,2,3],
        ret: vec![2,3]
      }
    ];

    for s in suites {
      let mut head = Some(Box::new(ListNode::new(s.head[0])));
      let mut head_mut = head.as_mut();
      for i in 1..s.head.len() {
        if let Some(node) = head_mut {
          node.next = Some(Box::new(ListNode::new(s.head[i])));
          head_mut = node.next.as_mut();
        }
      }
      let mut ret = Solution::delete_duplicates(head);
      let mut ret_mut = ret.as_mut();
      let mut arr = vec![];
      while let Some(node) = ret_mut {
        arr.push(node.val);
        ret_mut = node.next.as_mut();
      }
      assert_eq!(arr, s.ret);
    }
  }
}