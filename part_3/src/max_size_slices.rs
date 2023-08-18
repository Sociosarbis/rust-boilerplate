use super::*;

impl Solution {
  pub fn max_size_slices(slices: Vec<i32>) -> i32 {
    let k = slices.len() / 3;
    let max = |mut i: usize| {
      // 前i个中取了j个最大值，因为是环形，所以只能取到第n-1
      // 第i个能不能取，跟第i-2个取没取到无关，所以可以从dp[ii - 1][j - 1]递推
      let mut dp = vec![vec![0; k + 1]; slices.len()];
      dp[1][1] = slices[i];
      for ii in 1..slices.len() - 1 {
        i = (i + 1) % slices.len();
        for j in 1..=k {
          dp[ii + 1][j] = dp[ii][j].max(dp[ii - 1][j - 1] + slices[i]);
        }
      }
      dp[slices.len() - 1][k]
    };
    max(0).max(max(slices.len() - 1))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    slices: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_max_size_slices_simple() {
    let suites = vec![
      Suite {
        slices: vec![1, 2, 3, 4, 5, 6],
        ret: 10,
      },
      Suite {
        slices: vec![8, 9, 8, 6, 1, 1],
        ret: 16,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_size_slices(s.slices));
    }
  }
}
