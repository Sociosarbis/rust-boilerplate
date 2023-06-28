use super::*;

impl Solution {
  pub fn maximum_sum(arr: Vec<i32>) -> i32 {
    // 以i为结尾子串，在未作删减及删减过一次后的最大值
    let mut dp = vec![[0; 2]; arr.len()];
    dp[0][0] = arr[0];
    dp[0][1] = arr[0];
    let mut ret = arr[0];
    for (i, num) in arr.into_iter().enumerate().skip(1) {
      dp[i][0] = if dp[i - 1][0] > 0 { dp[i - 1][0] + num } else { num };
      dp[i][1] = (dp[i - 1][1] + num).max(dp[i - 1][0]);
      if dp[i][0] > ret {
        ret = dp[i][0];
      }
      if dp[i][1] > ret {
        ret = dp[i][1];
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_maximum_sum_simple() {
    let suites = vec![
      Suite {
        arr: vec![1, -2, 0, 3],
        ret: 4,
      },
      Suite {
        arr: vec![1, -2, -2, 3],
        ret: 3,
      },
      Suite {
        arr: vec![-1, -1, -1, -1],
        ret: -1,
      },
      Suite {
        arr: vec![1, -4, -5, -2, 5, 0, -1, 2],
        ret: 7,
      },
      Suite {
        arr: vec![8, 7, 10, 1, 0, 3, -10, 9, -4, 3, 10, -7, 8, -5],
        ret: 48,
      },
      Suite {
        arr: vec![11, -10, -11, 8, 7, -6, 9, 4, 11, 6, 5, 0],
        ret: 50,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::maximum_sum(s.arr));
    }
  }
}
