use super::Solution;

impl Solution {
  pub fn max_profit_4(k: i32, prices: Vec<i32>) -> i32 {
      let mut memo = vec![vec![0;prices.len()];k as usize];
      Solution::max_profit_dfs(k, &prices, 0, &mut memo)
  }

  pub fn max_profit_dfs(k: i32, prices: &Vec<i32>, from: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
      if k <= 0 || from >= prices.len() {
      return 0;
      }
      let mut max_ret = 0;
      let mut ret = 0;
      let mut min = prices[from];
      for i in from + 1..prices.len() {
          if prices[i] > min && prices[i] - min > ret {
              ret = prices[i] - min;
          } else {
              if ret != 0 {
                  let latter = ret + if memo[(k - 1) as usize][i] != 0 {
                      memo[(k - 1) as usize][i]
                  } else {
                      let tmp =  Solution::max_profit_dfs(k - 1, prices, i, memo);
                      memo[(k - 1) as usize][i] = tmp;
                      tmp
                  };
                  if latter > max_ret {
                      max_ret = latter;
                  }
              }
              if prices[i] < min {
                  min = prices[i];
                  ret = 0;
              }
          }
      }
      return if max_ret > ret { max_ret } else { ret };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    k: i32,
    prices: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn max_profit_4_simple() {
    let suites: Vec<Suite> = vec![
      Suite {
        k: 2,
        prices: vec![2, 4, 1],
        ret: 2,
      },
      Suite {
        k: 2,
        prices: vec![3,2,6,5,0,3],
        ret: 7,
      },
    ];

    for s in suites {
      assert_eq!(Solution::max_profit_4(s.k, s.prices), s.ret);
    }
  }
}