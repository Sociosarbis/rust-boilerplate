use super::*;

impl Solution {
  pub fn find_nth_digit(n: i32) -> i32 {
    let mut num = n as u32;
    let mut temp: u32 = 1;
    let mut count: u32 = 1;
    while num as u32 > count * temp * 9 {
      num -= count * temp * 9;
      temp *= 10;
      count += 1;
    }
    temp += num / count;
    if num % count == 0 {
      temp -= 1;
    }
    let mut i = count - (num - 1) % count - 1;
    while i != 0 {
      temp /= 10;
      i -= 1;
    }
    temp as i32 % 10
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    ret: i32
  }

  #[test]
  fn test_find_nth_digit_simple() {
    let suites = vec![
      Suite {
        n: 3,
        ret: 3
      },
      Suite {
        n: 11,
        ret: 0
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_nth_digit(s.n));
    }
  }
}