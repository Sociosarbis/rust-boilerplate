use super::*;

use std::collections::HashMap;

static M: i32 = 1000000007;

impl Solution {
  pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
    let grid: Vec<Vec<char>> = pizza.into_iter().map(|r| r.chars().collect()).collect();
    let mut dp = HashMap::new();
    let count = grid.iter().fold(0, |acc, row| {
      acc
        + row
          .iter()
          .fold(0, |acc, &cell| acc + if cell == 'A' { 1 } else { 0 })
    });
    if count < k {
      return 0;
    }
    if k == 1 {
      return 1;
    }
    Solution::ways_dfs(
      &grid,
      &mut dp,
      k as u8 - 1,
      0,
      0,
      grid[0].len() as u8,
      grid.len() as u8,
      count,
    )
  }

  fn ways_dfs(
    pizza: &Vec<Vec<char>>,
    dp: &mut HashMap<(u8, u8, u8, u8, u8), i32>,
    k: u8,
    i: u8,
    j: u8,
    w: u8,
    h: u8,
    count: i32,
  ) -> i32 {
    if k == 0 {
      return 1;
    }
    if let Some(&v) = dp.get(&(k, i, j, w, h)) {
      return v;
    }
    let mut ret = 0;
    let mut temp = 0;
    for ii in 0..h {
      for jj in 0..w {
        if pizza[(i + ii) as usize][(j + jj) as usize] == 'A' {
          temp += 1;
        }
      }
      if temp > 0 {
        if temp == count {
          break;
        } else {
          ret = (ret
            + Solution::ways_dfs(pizza, dp, k - 1, i + ii + 1, j, w, h - ii - 1, count - temp))
            % M;
        }
      }
    }
    temp = 0;
    for jj in 0..w {
      for ii in 0..h {
        if pizza[(i + ii) as usize][(j + jj) as usize] == 'A' {
          temp += 1;
        }
      }
      if temp > 0 {
        if temp == count {
          break;
        } else {
          ret = (ret
            + Solution::ways_dfs(pizza, dp, k - 1, i, j + jj + 1, w - jj - 1, h, count - temp))
            % M;
        }
      }
    }
    dp.insert((k, i, j, w, h), ret);
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    pizza: Vec<String>,
    k: i32,
    ret: i32,
  }

  #[test]
  fn test_ways_simple() {
    let suites = vec![
      Suite {
        pizza: t1!["A..", "AAA", "..."],
        k: 3,
        ret: 3,
      },
      Suite {
        pizza: t1!["A..", "AA.", "..."],
        k: 3,
        ret: 1,
      },
      Suite {
        pizza: t1!["A..", "A..", "..."],
        k: 1,
        ret: 1,
      },
      Suite {
        pizza: t1![".A", "AA", "A."],
        k: 3,
        ret: 3,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::ways(s.pizza, s.k));
    }
  }
}
