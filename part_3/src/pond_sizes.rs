use super::*;

impl Solution {
  pub fn pond_sizes(mut land: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ret = vec![];
    let m = land.len();
    let n = land[0].len();
    for i in 0..m {
      for j in 0..n {
        let temp = Solution::pond_sizes_dfs(&mut land, i, j);
        if temp > 0 {
          ret.push(temp);
        }
      }
    }
    ret.sort_unstable();
    ret
  }

  fn pond_sizes_dfs(land: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut ret = 0;
    if land[i][j] == 0 {
      land[i][j] = 1;
      ret += 1;
      let m = land.len();
      let n = land[0].len();
      if i > 0 {
        ret += Solution::pond_sizes_dfs(land, i - 1, j);
        if j > 0 {
          ret += Solution::pond_sizes_dfs(land, i - 1, j - 1);
        }
        if j + 1 < n {
          ret += Solution::pond_sizes_dfs(land, i - 1, j + 1);
        }
      }
      if i + 1 < m {
        ret += Solution::pond_sizes_dfs(land, i + 1, j);
        if j > 0 {
          ret += Solution::pond_sizes_dfs(land, i + 1, j - 1);
        }
        if j + 1 < n {
          ret += Solution::pond_sizes_dfs(land, i + 1, j + 1);
        }
      }
      if j > 0 {
        ret += Solution::pond_sizes_dfs(land, i, j - 1);
      }
      if j + 1 < n {
        ret += Solution::pond_sizes_dfs(land, i, j + 1);
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    land: Vec<Vec<i32>>,
    ret: Vec<i32>,
  }

  #[test]
  fn test_pond_sizes_simple() {
    let suites = vec![Suite {
      land: t2_i![[0, 2, 1, 0], [0, 1, 0, 1], [1, 1, 0, 1], [0, 1, 0, 1]],
      ret: vec![1, 2, 4],
    }];

    for s in suites {
      assert_eq!(s.ret, Solution::pond_sizes(s.land));
    }
  }
}
