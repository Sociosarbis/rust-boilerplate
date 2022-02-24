use super::*;

impl Solution {
  pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
    (0..grid[0].len()).into_iter().map(|i| Solution::find_ball_dfs(&grid, 0, i, 0)).collect()
  }

  fn find_ball_dfs(grid: &Vec<Vec<i32>>, mut i: usize, mut j: usize, mut dir: i32) -> i32 {
    if i >= grid.len() {
      return j as i32;
    }
    match grid[i][j]  {
      1 => {
        match dir {
          0 => {
            j += 1;
            if j >= grid[0].len() {
              return -1;
            }
            dir = 1;
          },
          1 => {
            i += 1;
            dir = 0;
          },
          2 => {
            return -1;
          },
          _ => {}
        }
      },
      -1 => {
        match dir {
          0 => {
            if j > 0 {
              j -= 1;
              dir = 2;
            } else {
              return -1;
            }
          },
          1 => {
            return -1;
          },
          2 => {
            i += 1;
            dir = 0;
          },
          _ => {}
        }
      },
      _ => {}
    }
    Solution::find_ball_dfs(grid, i, j, dir)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    grid: Vec<Vec<i32>>,
    ret: Vec<i32>
  }

  #[test]
  fn test_find_ball_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![[1,1,1,-1,-1],[1,1,1,-1,-1],[-1,-1,-1,1,1],[1,1,1,1,-1],[-1,-1,-1,-1,-1]],
        ret: vec![1,-1,-1,-1,-1]
      },
      Suite {
        grid: t2_i![[-1]],
        ret: vec![-1]
      },
      Suite {
        grid: t2_i![[1,1,1,1,1,1],[-1,-1,-1,-1,-1,-1],[1,1,1,1,1,1],[-1,-1,-1,-1,-1,-1]],
        ret: vec![0,1,2,3,4,-1]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_ball(s.grid));
    }
  }
}