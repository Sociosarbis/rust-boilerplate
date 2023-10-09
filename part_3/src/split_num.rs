use super::*;

impl Solution {
  pub fn split_num(mut num: i32) -> i32 {
    let mut counter = [0;10];
    while num != 0 {
      counter[(num % 10) as usize] += 1;
      num /= 10;
    }
    let mut ret = 0;
    let mut i = 9;
    let mut base = 1;
    'a: loop {
      for _ in 0..2 {
        while counter[i] == 0 {
          if i > 0 {
            i -= 1;
          } else {
            break 'a;
          }
        }
        ret += i as i32 * base;
        counter[i] -= 1;
      }
      base *= 10;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    num: i32,
    ret: i32,
  }

  #[test]
  fn test_split_num_simple() {
    let suites = vec![Suite { num: 4325, ret: 59 }, Suite { num: 687, ret: 75 }];

    for s in suites {
      assert_eq!(s.ret, Solution::split_num(s.num));
    }
  }
}
