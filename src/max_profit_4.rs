use super::Solution;

impl Solution {
  pub fn max_profit_4(k: i32, prices: Vec<i32>) -> i32 {
      Solution::max_profit_dfs(k, &prices, 0)
  }

  pub fn max_profit_dfs(k: i32, prices: &Vec<i32>, from: usize) -> i32 {
    if k <= 0 || from >= prices.len() {
      return 0;
    }
    let mut max_ret = 0;
    let mut ret = 0;
    let mut min = prices[from];
    for i in from + 1..prices.len() {
      if prices[i] > min {
        if prices[i] - min > ret {
          ret = prices[i] - min;
        }
      } else if prices[i] < min {
        if ret != 0 {
          let latter = ret + Solution::max_profit_dfs(k - 1, prices, i);
          if latter > max_ret {
            max_ret = latter;
          }
        }
        min = prices[i];
        ret = 0;
      }
    }
    return if max_ret > ret { max_ret } else { ret };
  }
}