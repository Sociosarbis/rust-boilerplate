struct Solution {}


impl Solution {
  #[allow(dead_code)]
  pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let mut profit = 0;
    let mut keep_stock = 0;
    let mut prev_profit = 0;
    let mut prev_keep_stock = 0;
    let mut ret = 0;
    for p in prices {
      if (keep_stock == 0 || keep_stock > p || p - keep_stock <= fee) &&
        prev_keep_stock > 0 && p - fee + prev_profit > ret {
        profit = p - fee + prev_profit;
        ret = profit;
        keep_stock = 0;
      } else {
        if keep_stock == 0 {
          keep_stock = p;
          profit -= p;
        } else if keep_stock > p {
          profit += keep_stock - p;
          keep_stock = p;
        } else if p - keep_stock > fee {
          let new_profit = profit + p - fee;
          if prev_keep_stock > 0 && p - fee + prev_profit > new_profit {
            profit = p - fee + prev_profit;
          } else {
            prev_keep_stock = keep_stock;
            prev_profit = profit;
            profit = new_profit;
          }
          ret = profit;
          keep_stock = 0;
        }
      }
    }
    ret
  }
}

mod tests {
  use super::*;

  #[test]
  fn max_profit_simple() {
    assert_eq!(Solution::max_profit(vec![6, 3, 5, 2, 8, 1, 5, 9, 5, 6], 2), 10);
  }
}