use super::*;

use std::{cmp::Reverse, collections::BinaryHeap};

enum State {
  Left,
  CrossBridgeLeft,
  Right,
  CrossBridgeRight,
}

impl From<u8> for State {
  fn from(v: u8) -> State {
    match v {
      0 => Self::Left,
      1 => Self::CrossBridgeLeft,
      2 => Self::Right,
      _ => Self::CrossBridgeRight,
    }
  }
}

impl State {
  fn next(self) -> State {
    ((self as u8 + 1) % 4).into()
  }
}

impl Solution {
  pub fn find_crossing_time(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
    // 转到下个状态的时间，序号，状态
    let mut queue: BinaryHeap<(Reverse<i32>, usize, u8)> = BinaryHeap::new();
    let mut left_queue: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    let mut right_queue: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    let mut bridge_used = true;
    for i in 0..k as usize {
      left_queue.push((time[i][0] + time[i][2], i));
    }
    let mut ret = 0;
    if let Some((_, i)) = left_queue.pop() {
      queue.push((Reverse(time[i][0]), i, State::CrossBridgeLeft as u8));
    }
    let mut cross_left = 0;
    let mut cross_right = 0;
    while let Some((Reverse(t), i, state)) = queue.pop() {
      ret = t;
      let mut items = vec![(i, state)];
      while let Some((Reverse(t1), i1, state1)) = queue.pop() {
        if t1 != ret {
          queue.push((Reverse(t1), i1, state1));
          break;
        } else {
          items.push((i1, state1));
        }
      }
      for (i, state) in items {
        match state.into() {
          s @ State::CrossBridgeLeft => {
            bridge_used = false;
            cross_left += 1;
            // 如果没有箱子，则取消左边的队伍
            if cross_left == n {
              left_queue.clear();
            }
            queue.push((Reverse(ret + time[i][1]), i, s.next() as u8));
          }
          s @ State::CrossBridgeRight => {
            bridge_used = false;
            cross_right += 1;
            if cross_right == n {
              return ret;
            }
            queue.push((Reverse(ret + time[i][3]), i, s.next() as u8));
          }
          State::Left => {
            // 如果还需要拿箱子，就排队过桥
            if cross_left < n {
              left_queue.push((time[i][0] + time[i][2], i));
            }
          }
          State::Right => {
            right_queue.push((time[i][0] + time[i][2], i));
          }
        }
      }
      if !bridge_used {
        if let Some((_, i)) = right_queue.pop() {
          bridge_used = true;
          queue.push((Reverse(ret + time[i][2]), i, State::CrossBridgeRight as u8));
        } else if let Some((_, i)) = left_queue.pop() {
          bridge_used = true;
          queue.push((Reverse(ret + time[i][0]), i, State::CrossBridgeLeft as u8));
        }
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    k: i32,
    time: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_find_crossing_time_simple() {
    let suites = vec![
      Suite {
        n: 1,
        k: 3,
        time: t2_i![[1, 1, 2, 1], [1, 1, 3, 1], [1, 1, 4, 1]],
        ret: 6,
      },
      Suite {
        n: 3,
        k: 2,
        time: t2_i![[1, 9, 1, 8], [10, 10, 10, 10]],
        ret: 50,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_crossing_time(s.n, s.k, s.time))
    }
  }
}
