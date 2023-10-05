use super::*;

impl Solution {
  pub fn max_profit_2(prices: Vec<i32>) -> i32 {
    let mut max_sell = [0; 2];
    let mut max_buy = -prices[0];
    for p in prices.into_iter().skip(1) {
      let next_buy = (max_sell[0] - p).max(max_buy);
      max_sell[0] = max_sell[0].max(max_sell[1]);
      max_sell[1] = p + max_buy;
      max_buy = next_buy;
    }
    max_sell[0].max(max_sell[1])
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
        prices: vec![1, 2, 3, 0, 2],
        ret: 3,
      },
      Suite {
        prices: vec![1],
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_profit_2(s.prices));
    }
  }
}
