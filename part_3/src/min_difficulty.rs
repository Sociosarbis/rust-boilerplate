use super::*;

impl Solution {
  pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
    let n = job_difficulty.len();
    if (n as i32) < d {
      return -1;
    }
    let mut dp = vec![vec![0; n + 1]; d as usize + 1];
    // dp[i][j] 表示i日以j为终点的最小值
    for day in 1..=d as usize {
      for task in day..=n {
        let mut local_max = job_difficulty[task - 1];
        for s in (day..=task).rev() {
          local_max = local_max.max(job_difficulty[s - 1]);
          if dp[day - 1][s - 1] != 0 || (day == 1 && s == 1) {
            if dp[day][task] == 0 {
              dp[day][task] = dp[day - 1][s - 1] + local_max;
            } else {
              dp[day][task] = dp[day][task].min(dp[day - 1][s - 1] + local_max);
            }
          }
        }
      }
    }
    dp[d as usize][n]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    job_difficulty: Vec<i32>,
    d: i32,
    ret: i32,
  }

  #[test]
  fn test_min_difficulty_simple() {
    let suites = vec![
      Suite {
        job_difficulty: vec![6, 5, 4, 3, 2, 1],
        d: 2,
        ret: 7,
      },
      Suite {
        job_difficulty: vec![9, 9, 9],
        d: 4,
        ret: -1,
      },
      Suite {
        job_difficulty: vec![1, 1, 1],
        d: 3,
        ret: 3,
      },
      Suite {
        job_difficulty: vec![
          143, 446, 351, 170, 117, 963, 785, 76, 139, 772, 452, 743, 23, 767, 564, 872, 922, 532,
          957, 945, 203, 615, 843, 909, 458, 320, 290, 235, 174, 814, 414, 669, 422, 769, 781, 721,
          523, 94, 100, 464, 484, 562, 941,
        ],
        d: 5,
        ret: 1839,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_difficulty(s.job_difficulty, s.d));
    }
  }
}
