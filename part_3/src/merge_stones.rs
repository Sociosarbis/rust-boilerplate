use super::*;

impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = stones.len();
        if stones.len() == 1 {
          return 0
        }
        if stones.len() < k {
          return -1;
        }
        if k > 2 {
            if stones.len() % (k - 1) != 1 {
                return -1;
            }
        }
        let mut sums = vec![0;n + 1];
        for i in 0..n {
          sums[i + 1] = sums[i] + stones[i];
        }
        // dp表示将数字尽可能合并以后的最小值
        let mut dp = vec![vec![0;n as usize];n as usize];
        for len in k..=n {
          for i in 0..=n - len {
            let j = i + len - 1;
            // 先保证[i,t]是能合成一堆的，因为一开始没有算长度小于k的dp并且合并的前提必然是至少要有1堆
            for t in (i..j).step_by(k - 1) {
              let temp = dp[i][t] + dp[t + 1][j];
              if dp[i][j] == 0 {
                dp[i][j] = temp;
              } else {
                dp[i][j] = dp[i][j].min(temp);
              }
            }
            // 当区间是能合并为1堆时，要再把所有的数加一次
            if (j - i) % (k - 1) == 0 {
              dp[i][j] += sums[j + 1] - sums[i];
            }
          }
        }
        dp[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        stones: Vec<i32>,
        k: i32,
        ret: i32,
    }

    #[test]
    fn test_merge_stones_simple() {
        let suites = vec![
            Suite {
                stones: vec![3, 2, 4, 1],
                k: 2,
                ret: 20,
            },
            Suite {
                stones: vec![3, 2, 4, 1],
                k: 3,
                ret: -1,
            },
            Suite {
                stones: vec![3, 5, 1, 2, 6],
                k: 3,
                ret: 25,
            },
            Suite {
              stones: vec![16,43,87,30,4,98,12,30,47,45,32,4,64,14,24,84,86,51,11,22,4],
              k: 2,
              ret: 3334
            }
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::merge_stones(s.stones, s.k));
        }
    }
}
