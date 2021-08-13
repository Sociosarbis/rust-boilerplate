use super::Solution;

use std::collections::HashMap;

impl Solution {
  pub fn count_digit_one(n: i32) -> i32 {
    if n == 0 {
      return 0;
    }
    if n < 10 {
      return 1;
    }
    let mut base = 1;
    let mut num = n / 10;
    let mut dp: HashMap<i32, i32> = HashMap::new();
    dp.insert(0, 0);
    for i in 1..10 {
      dp.insert(i, 1);
    }
    let mut count = n % 10;
    while num > 0 {
      let c = num % 10;
      num /= 10;
      base *= 10;
      let rest = count;
      count += c * base;
      if !dp.contains_key(&count) {
        dp.insert(count, c * dp.get(&(base - 1)).unwrap() + dp.get(&rest).unwrap() + if c > 1 { base } else { rest + 1 });
      }
      if num > 0 {
        dp.insert(base * 10 - 1, 10 * dp.get(&(base - 1)).unwrap() + base);
      }
    }
    *dp.get(&n).unwrap()
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
  fn test_count_digit_one_simple() {
    let suites = vec![
      Suite {
        n: 13,
        ret: 6
      },
      Suite {
        n: 0,
        ret: 0
      },
      Suite {
        n: 1000000000,
        ret: 900000001
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_digit_one(s.n));
    }
  }
}