use super::*;

impl Solution {
  pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
    if hand.len() as i32 % group_size != 0 {
      return false;
    }
    hand.sort_unstable();
    let mut queue = vec![];
    let mut i = 0;
    while i < hand.len() {
      let mut num = (hand[i], 1);
      while i + 1 < hand.len() && hand[i + 1] == num.0 {
        num.1 += 1;
        i += 1;
      }
      queue.push(num);
      i += 1;
    }
    i = 0;
    let mut min = 0;
    while i < queue.len() - group_size as usize + 1 {
      let mut j = i;
      while j < queue.len() && j - i < group_size as usize {
        if j == i {
          min = queue[j].1;
        } else {
          if queue[j].1 < min || queue[j].0 != queue[j - 1].0 + 1 {
            return false;
          }
        }
        j += 1;
      }
      if j - i != group_size as usize {
        return false;
      }
      for k in i..j {
        queue[k].1 -= min;
      }
      while i < queue.len() {
        if queue[i].1 == 0 {
          i += 1;
        } else {
          break;
        }
      }
    }
    i == queue.len()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    hand: Vec<i32>,
    group_size: i32,
    ret: bool
  }

  #[test]
  fn test_is_n_straight_hand_simple() {
    let suites = vec![
      Suite {
        hand: vec![1,2,3,6,2,3,4,7,8],
        group_size: 3,
        ret: true
      },
      Suite {
        hand: vec![1,2,3,4,5],
        group_size: 4,
        ret: false
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::is_n_straight_hand(s.hand, s.group_size));
    }
  }
}