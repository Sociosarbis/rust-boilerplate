use super::*;

impl Solution {
  pub fn punishment_number(n: i32) -> i32 {
    (1..=n).into_iter().fold(0, |acc, num| {
      let temp = num * num;
      if Solution::punishment_number_check(temp, num) {
        acc + temp
      } else {
        acc
      }
    })
  }

  fn punishment_number_check(num: i32, target: i32) -> bool {
    if num == target {
      return true;
    }
    let mut base = 10;
    while num / base != 0 {
      let temp = num % base;
      if temp > target {
        break;
      }
      if Solution::punishment_number_check(num / base, target - temp) {
        return true;
      }
      base *= 10;
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    ret: i32,
  }

  #[test]
  fn test_punishment_number_simple() {
    let suites = vec![Suite { n: 10, ret: 182 }, Suite { n: 37, ret: 1478 }];

    for s in suites {
      assert_eq!(s.ret, Solution::punishment_number(s.n));
    }
  }
}
