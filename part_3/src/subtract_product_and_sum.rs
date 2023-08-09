use super::*;

impl Solution {
  pub fn subtract_product_and_sum(mut n: i32) -> i32 {
    let mut a = 1;
    let mut b = 0;
    while n != 0 {
      a *= n % 10;
      b += n % 10;
      n /= 10;
    }
    a - b
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
  fn test_subtract_product_and_sum_simple() {
    let suites = vec![Suite { n: 234, ret: 15 }, Suite { n: 4421, ret: 21 }];

    for s in suites {
      assert_eq!(s.ret, Solution::subtract_product_and_sum(s.n));
    }
  }
}
