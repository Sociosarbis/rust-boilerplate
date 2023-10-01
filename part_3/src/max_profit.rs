use super::*;

impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    let first_value = prices[0];
    prices
      .into_iter()
      .skip(1)
      .fold((0, first_value), |(ret, min), p| {
        ((p - min).max(ret), min.min(p))
      })
      .0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    prices: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_max_profit_simple() {
    let suites = vec![
      Suite {
        prices: vec![7, 1, 5, 3, 6, 4],
        ret: 5,
      },
      Suite {
        prices: vec![7, 6, 4, 3, 1],
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_profit(s.prices));
    }
  }
}
