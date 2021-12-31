use super::*;

impl Solution {
  pub fn check_perfect_number(num: i32) -> bool {
    if num == 1 {
      return false;
    }
    let max = (num as f32).sqrt() as i32;
    let mut remain = num - 1;
    for i in 2..max + 1 {
      if num % i == 0 {
        remain -= i + (num / i);
      }
    }
    remain == 0
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    num: i32,
    ret: bool
  }

  #[test]
  fn test_check_perfect_number_simple() {
    let suites = vec![
      Suite {
        num: 28,
        ret: true
      },
      Suite {
        num: 7,
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::check_perfect_number(s.num));
    }
  }
}