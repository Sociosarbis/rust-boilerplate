use super::*;

impl Solution {
  pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
    let n = 1 << cost[0].len();
    let mut dp = vec![vec![-1; n]; 2];
    for i in 0..cost.len() {
      Solution::connect_two_groups_dfs(&mut dp, &cost, i, 0, 0, 0);
      let prev_index = 1 - (i % 2);
      dp[prev_index].fill(-1);
    }
    dp[(cost.len() - 1) % 2][n - 1]
  }

  fn connect_two_groups_dfs(
    dp: &mut Vec<Vec<i32>>,
    cost: &Vec<Vec<i32>>,
    i: usize,
    mask: usize,
    temp: i32,
    j: usize,
  ) {
    if j >= cost[i].len() {
      return;
    }
    let next_temp = temp + cost[i][j];
    let next_mask = mask | 1 << j;
    let index = i % 2;
    let prev_index = 1 - (i % 2);
    if i == 0 {
      dp[index][next_mask] = next_temp;
    } else {
      for k in 1..dp[prev_index].len() {
        let next_value = dp[prev_index][k].max(0) + next_temp;
        let next_k = k | next_mask;
        if dp[index][next_k] == -1 || next_value < dp[index][next_k] {
          dp[index][next_k] = next_value;
        }
      }
    }
    Solution::connect_two_groups_dfs(dp, cost, i, next_mask, next_temp, j + 1);
    Solution::connect_two_groups_dfs(dp, cost, i, mask, temp, j + 1);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    cost: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_connect_two_groups_simple() {
    let suites = vec![
      Suite {
        cost: t2_i![[15, 96], [36, 2]],
        ret: 17,
      },
      Suite {
        cost: t2_i![[1, 3, 5], [4, 1, 1], [1, 5, 3]],
        ret: 4,
      },
      Suite {
        cost: t2_i![[2, 5, 1], [3, 4, 7], [8, 1, 2], [6, 2, 4], [3, 8, 8]],
        ret: 10,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::connect_two_groups(s.cost));
    }
  }
}
