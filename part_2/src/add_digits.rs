use super::*;

impl Solution {
  pub fn add_digits(mut num: i32) -> i32 {
    while num >= 10 {
      let mut temp = 0;
      while num != 0 {
        temp += num % 10;
        num /= 10;
      }
      num = temp;
    }
    num
  }

  pub fn add_digits_best(num: i32) -> i32 {
    if num == 0 {
        0
    } else {
      (num - 1) % 9 + 1
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    num: i32,
    ret: i32
  }

  #[test]
  fn test_add_digits_simple() {
    let suites = vec![
      Suite {
        num: 38,
        ret: 2
      },
      Suite {
        num: 0,
        ret: 0
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::add_digits(s.num));
    }
  }
}