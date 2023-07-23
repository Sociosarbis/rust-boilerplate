use super::*;

impl Solution {
  pub fn trap(height: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut stack: Vec<(i32, i32)> = vec![];
    for h in height {
      if !stack.is_empty() {
        let shorter_h = stack[0].0.min(h);
        let mut n = stack.len();
        let mut count = 0;
        while n > 0 && stack[n - 1].0 <= shorter_h {
          ret += (shorter_h - stack[n - 1].0) * stack[n - 1].1;
          count += stack[n - 1].1;
          n -= 1;
        }
        if n == 0 {
          stack.clear();
          stack.push((h, 1));
        } else {
          stack.drain(n..);
          stack.push((h, count + 1));
        }
      } else {
        if h != 0 {
          stack.push((h, 1));
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
    height: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_trap_simple() {
    let suites = vec![
      Suite {
        height: vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
        ret: 6,
      },
      Suite {
        height: vec![4, 2, 0, 3, 2, 5],
        ret: 9,
      },
      Suite {
        height: vec![5,5,1,7,1,1,5,2,7,6],
        ret: 23
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::trap(s.height));
    }
  }
}
